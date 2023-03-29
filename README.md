# GitHub DCO  Reminder


[Deploy this function on flows.network](#deploy-dco-reminder-app-on-your-github-repo), and you will get a GitHub bot that remindering the contributor if the DCO test failed. It saves maintainers' time to review a PR!

<img width="924" alt="image" src="https://user-images.githubusercontent.com/45785633/227521014-66168f85-a5b4-4ec6-98ec-210f2c87eb88.png">


## Deploy DCO Reminder App on your GitHub repo

To install the DCO Reminder GitHub App, we will use [flows.network](https://flows.network/), a serverless platform that makes deploying your own app quick and easy in just three steps.

### Fork this repo

Fork [this repo](https://github.com/flows-network/github-dco-reminder/) and customize the code based on your needs. 

<img width="637" alt="image" src="https://user-images.githubusercontent.com/45785633/227510525-f3a231b9-d888-41d3-ae89-4288e75e5adc.png">

### Deploy the code on flow.network

1. Sign up for an account for deploying flows on [flows.network](https://flows.network/). It's free.
2. Click on the "Create a Flow" button to start deploying the GitHub DCO Reminder APP
3. Authenticate the [flows.network](https://flows.network/) to access the `github-dco-reminder` repo you just forked. 

<img width="860" alt="image" src="https://user-images.githubusercontent.com/45785633/227522025-64cb6dbc-4cdf-4fbf-8152-015393effcdc.png">

4. click the Deploy button to deploy your function.

### Configure SaaS integrations

After that, the flows.network will direct you to configure the SaaS integration required by your flow. Here we can see, we need to configue one SaaS integration: GitHub.

<img width="764" alt="image" src="https://user-images.githubusercontent.com/45785633/227522621-b04f5ade-21c1-4fa3-95b7-0e00d190dc3f.png">


Click the "Connect/+ Add new authentication" button to authenticate your GitHub account. You'll be redirected to a new page where you must grant [flows.network](https://flows.network/) permission to install the `flows-network-integration` bot on a repo. This repo is the one you entered into the environment variables above.

After that, click the Check button to see your flow details. As soon as the flow function's status becomes `ready` and the flow's status became `running`, the GitHub DCO Reminder App goes live. 

<img width="1137" alt="image" src="https://user-images.githubusercontent.com/45785633/227523068-6e460c74-aaea-4ecd-a29b-cf6691920962.png">

> [flows.network](https://flows.network/) is still in its early stages. We would love to hear your feedback!







