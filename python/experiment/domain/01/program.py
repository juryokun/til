from user import User
from username import UserName
from iuserrepository import IUserRepository
from userservice import UserService
from injector import Injector, inject, Module
from inmemoryuserrepository import InMemoryUserRepository


class Program():

    @inject
    def __init__(self, user_repository: IUserRepository):
        if not isinstance(user_repository, IUserRepository):
            raise Exception("user_repository is not IUserRepository")

        self._user_repository = user_repository

    def create_user(self, user_name: str):
        user = User(UserName(user_name))

        user_service = UserService(self._user_repository)
        if user_service.exists(user):
            raise(user + "は既に存在しています。")

        user_service.save(user)


class ProgramDIModule(Module):
    def configure(self, binder):
        binder.bind(IUserRepository, to=InMemoryUserRepository)


if __name__ == '__main__':
    injector = Injector([ProgramDIModule()])
    program: Program = injector.get(Program)
    print(type(program._user_repository))
