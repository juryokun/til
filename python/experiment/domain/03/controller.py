from bottle import route, run, get, request, Bottle
from userapplicationservice import UserApplicationService
from injector import inject

bottle = Bottle()


class UserController():

    @inject
    def __init__(self, user_application_service: UserApplicationService):
        self._user_application_service = user_application_service
