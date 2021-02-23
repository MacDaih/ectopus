# ectopus
A backend web service to store and serve Cypress io e2e tests reports.

#Install rust

https://www.rust-lang.org/tools/install

# Run

Once you are all set, the project can be run with ```cargo run```

# Settings

This project uses MongoDB to store reports. You need to install and configure MongoDB properly before running this app. Moreover, this code stands as a demo, thus it does not implement authentication flow to DB.

https://docs.mongodb.com/manual/installation/

# Sending a report

Once the service is running, you can provide report manually to the webservice with :

```curl -i -X POST http://localhost:8000/new -H "Content-type: application/json" -d @report.json```
