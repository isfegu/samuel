import unittest
import dotpyphen

class TestDotPyphen(unittest.TestCase):
  def test_translate(self):
    self.assertEqual(dotpyphen.translate("Hello World"), ".... . .-.. .-.. --- / .-- --- .-. .-.. -..")