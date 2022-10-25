"""
To test:

docker container run --rm --network=host --name locust -w /locust -v ${PWD}/locust:/locust locustio/locust -f example.py --headless --users 1000 --spawn-rate 100 -t 10s -H http://localhost:3000

"""

import itertools
import time
from datetime import datetime
from locust import HttpUser, task, between

class QuickstartUser(HttpUser):
    wait_time = between(1, 5)
    id_notes = itertools.count(0,1)

    @task
    def create_notes(self):
        newid = next(self.id_notes)

        with self.client.post("/", json={"title":"note {}".format(newid), "content": "data {} - {}".format(newid, datetime.now()) }, catch_response=True) as response:
            if response.status_code == 201:
                response.success()
            else:
                response.failure("")

    @task
    def get_all(self):
        with self.client.get("/", catch_response=True) as response:
            if response.status_code == 200:
                response.success()
            else:
                response.failure("")