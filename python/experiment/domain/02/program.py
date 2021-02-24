from user import User
from username import UserName
from iuserrepository import IUserRepository
from userservice import UserService
from injector import Injector, inject, Module, singleton
from inmemoryuserrepository import InMemoryUserRepository
from userapplicationservice import UserApplicationService
from configuration import TestConfigration


class Program():

    def main(self, args: [str] = None):
        self.startup()
        while True:
            print("Input user name")
            print(">", end="")
            input_str = input().strip()

            user_application_service = self.injector.get(
                UserApplicationService)
            user_application_service.register(input_str)

            print("--------------------")
            print("user created.")
            print("--------------------")
            print("continue?(y/n)")
            print(">", end="")
            yes_or_no = input()
            if yes_or_no == "n":
                break

    def startup(self):
        self.injector = Injector([TestConfigration()])


if __name__ == '__main__':
    program = Program()
    program.main()
#     injector = Injector([ProgramDIModule()])
#     program: Program = injector.get(Program)
#     print(type(program._user_repository))
