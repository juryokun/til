import unittest
from program import Program
from inmemoryuserrepository import InMemoryUserRepository
from iuserrepository import IUserRepository
from injector import Injector, inject, Module


class TestProgram(unittest.TestCase):
    def test_create_user(self):
        test_user = "yamamoto"
        injector = Injector([ProgramDIModule()])
        program: Program = injector.get(Program)
        program.create_user(test_user)
        t = None
        for i in program._user_repository.store.values():
            if i.name.user_name == test_user:
                t = i.name.user_name
        self.assertEqual(test_user, t)


class ProgramDIModule(Module):
    def configure(self, binder):
        binder.bind(IUserRepository, to=InMemoryUserRepository)


if __name__ == '__main__':
    unittest.main()
