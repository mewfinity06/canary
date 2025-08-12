// Variables!

// Mutable variable, explicit type
mut foo : i32 = 69;

// Immutable variable, implicit type
//   castable to 'mut'
let bar := 420;

// Const variable, type must be known at compile time
//   not castable to 'mut'
const Baz := str = "I am a string!s";

// Static variable, type must be known at compile time
//   immutable, castable to 'mut'
static Tau : f16 = Pi*2;
//   mutable
static mut Pi : f16 = 3.14159;
//   constant, not castable to 'mut'
static const Tau : f16 = Pi*2; 

// Enums!
const Activity : enum {
  Todo: str,
  SwimLaps: u8,
  Meditate: struct = {
    times: u8,
    seconds: f16,
  },
  AllDone,
};

const swim := Activity.SwimLaps(8);
const meditate: Activity = .Meditate {
  .times = 6,
  .seconds = 10.0,
};

Activity += impl {
  const do_activity : fn(&self) -> void = {
    switch (self) {
      .Todo : |todo| => { printf("TODO: {s}\n", todo); },
      .SwimLaps : |laps| => printf("Swim {d} laps\n", laps),
      .Meditate : |{times, seconds}| => {
         printf("Meditate {d} times for {f} seconds\n", times, seconds);
      },
      _ => return,
    }
  };
};

// Structs!
const Person : struct = {
  name: str,
  age : u8,
};

// Deriving an interface!
Person += PrettyPrint{};

// Person member functions and the such
Person += impl {
  pub const new : fn(name: str, age: u8) -> Self = {.{
      .name = name,
      .age = age,
  }};

  pub const who_am_i : fn(&self) -> void = {
    printf("My name is {s} and I am {d} years old!\n", self.name, self.age);
  };
};