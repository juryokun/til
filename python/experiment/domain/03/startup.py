from configuration import TestConfigration
from bottle import Bottle
from injector import Injector, inject
from controller import UserController
# from iconfiguration import IConfiguration


class Startup():
    def __init__(self, configuration: TestConfigration):
        self.injector = Injector([conf])


conf = TestConfigration()
startup = Startup(conf)
main: Bottle = Bottle()


@main.route()
def post(request):
    user_controller = startup.injector.get(UserController)
    pass


if __name__ == '__main__':
    main.run(host='0.0.0.0', port=8000, debug=True, reloader=True)
