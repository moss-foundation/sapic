import wit_world
from wit_world.imports.types import Value, Value_Str
from wit_world.imports.host_functions import greet


class WitWorld(wit_world.WitWorld):
    def execute(self, input: Value) -> Value:
        greet(input)
        return_val = Value_Str("Success")
        return return_val