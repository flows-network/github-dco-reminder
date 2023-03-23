use github_flows::octocrab::models::repos::RepoCommit;
use github_flows::{
    get_octo, listen_to_event,
    octocrab::{models::events::payload::PullRequestEventAction, Result as OctoResult},
    EventPayload,
};
use lazy_static::lazy_static;
use regex::Regex;
use tokio::*;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() -> anyhow::Result<()> {
    let owner = "WasmEdge";
    let repo = "WasmEdge";

    listen_to_event(owner, repo, vec!["pull_request"], |payload| {
        handler(owner, repo, payload)
    })
    .await;

    Ok(())
}

async fn handler(owner: &str, repo: &str, payload: EventPayload) {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r#"Signed-off-by: \w+ <[\w._%+-]+@[\w.-]+\.\w{2,4}>"#).unwrap();
    }

    let octocrab = get_octo(Some(String::from(owner)));
    let mut pull = None;

    match payload {
        EventPayload::PullRequestEvent(e) => {
            if e.action != PullRequestEventAction::Opened {
                return;
            }
            pull = Some(e.pull_request);
        }

        _ => (),
    };

    let (commits_url, pull_number, creator) = match pull {
        Some(p) => (p.commits_url.unwrap(), p.number, p.user.unwrap().login),
        None => return,
    };

    let query_str = commits_url
        .path_segments()
        .unwrap()
        .collect::<Vec<&str>>()
        .join("/");

    let mut is_dco_ok = false;

    let response: OctoResult<Vec<RepoCommit>> = octocrab.get(query_str, None::<&()>).await;
    match response {
        Err(_) => {}
        Ok(arr) => {
            is_dco_ok = arr
                .iter()
                .map(|j| {
                    let msg = j.commit.message.lines().last().unwrap_or_default();
                    RE.is_match(msg)
                })
                .all(std::convert::identity);
        }
    };

    let msg: &str = if is_dco_ok {
        "Thanks for your contribution! The miantainers will review your PR soon."
    } else {
        "Thanks for your contributin. It seems that your DCO test is failed. Please fix it.'"
    };
    let body = format!("@{creator}, {msg}");
    let _ = octocrab
        .issues(owner, repo)
        .create_comment(pull_number, body)
        .await;
}
