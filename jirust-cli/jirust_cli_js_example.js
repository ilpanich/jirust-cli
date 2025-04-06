import { run } from "./pkg/jirust_cli.js";

let cmd = run(["jirust-cli", "project", "list"], {
  auth: {
    auth_token: "jira_auth_token",
  },
  jira: {
    jira_url: "https://jira.atlassian.net",
    standard_resolution: '{"name": "Done"}',
    standard_resolution_comment: "Autoresolved",
    transitions_names: { resolve: ["Resolve Issue"] },
  },
}).then((v, err) => {
  if (err) {
    console.error(err);
  } else {
    console.log(JSON.stringify(v));
    console.log("Done");
  }
});
