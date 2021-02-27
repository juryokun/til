from injector import Injector, inject, Module, singleton
from inmemoryuserrepository import InMemoryUserRepository
from userapplicationservice import UserApplicationService
from iuserrepository import IUserRepository
from userservice import UserService


class TestConfigration(Module):
    def configure(self, binder):
        binder.bind(IUserRepository,
                    to=InMemoryUserRepository, scope=singleton)
        binder.bind(UserService, to=UserService)
        binder.bind(UserApplicationService, to=UserApplicationService)
