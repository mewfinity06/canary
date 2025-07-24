
const printf := std.c.printf;

// Struct definition for a 'User'
const User : struct = {
    username: str,
    email: str,
    is_active: bool,
};

// Enum for user roles
const UserRole : enum = {
    Admin,
    Moderator,
    Subscriber,
    Guest,
};

// A simple macro for logging messages
const log : macro = {
    (msg: str) := {
        printf("[LOG] {s}%n", #msg);
    };
};

// Implementation block for the 'User' struct
User += impl {
    // A function to create a new user
    pub const new : fn(username: str, email: str) -> User = {
        return .{
            .username = username,
            .email = email,
            .is_active = true,
        };
    };

    // A function to greet the user
    pub const greet : fn(self) -> void = {
        log!("Greeting user...");
        printf("Hello, {s}!%n", self.username);
    };
};

// The main entry point of the program
const main : fn() -> void = {
    // Create a new user
    const user := User.new("CanaryUser", "user@example.com");

    // Greet the user
    user.greet();

    // Set a role for the user
    const role := UserRole.Admin;

    // Check the user's role and print a message
    if (user.is_active) {
        switch (role) {
            .Admin: |admin| => {
                printf("Welcome, admin!%n");
            },
            .Guest: |guest| => {
                printf("Welcome, guest!%n");
            },
            else => {
                printf("Welcome, user!%n");
            }
        };
    } else {
        printf("User is not active.%n");
    }
};
