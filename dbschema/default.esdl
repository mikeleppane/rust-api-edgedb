module default {
    type User {
      required name: str;
      required email: str {
        constraint exclusive;
      }
    }
}
