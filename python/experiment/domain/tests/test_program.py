import unittest
from program import Program
from inmemoryuserrepository import InMemoryUserRepository


class TestProgram(unittest.TestCase):
    def test_create_user(self):
        test_user = "yamamoto"
        user_repository = InMemoryUserRepository()
        program = Program(user_repository)
        program.create_user(test_user)
        t = None
        for i in user_repository.store.values():
            if i.user_name.user_name == test_user:
                t = i.user_name.user_name
        self.assertEqual(test_user, t)


if __name__ == '__main__':
    unittest.main()
