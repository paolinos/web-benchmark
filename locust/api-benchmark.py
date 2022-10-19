import itertools
import time
from datetime import datetime
from locust import HttpUser, task, between

class QuickstartUser(HttpUser):
    wait_time = between(1, 5)
    id_notes = itertools.count(0,1)

    def on_start(self):
        self.client.headers.update({'userid': 'some@user.com'})
    
    @task
    def health(self):
        self.client.get("/health")

    @task
    def create_notes(self):
        newid = next(self.id_notes)

        with self.client.post("/note", json={"title":"note {}".format(newid), "content": "data {} - {}".format(newid, datetime.now()) }, catch_response=True) as response:
            if response.status_code == 201:
                response.success()
            else:
                response.failure("")

    @task
    def get_all(self):
        with self.client.get("/note", catch_response=True) as response:
            if response.status_code == 200:
                response.success()
            else:
                response.failure("")