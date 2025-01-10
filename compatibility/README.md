# Bincode compatibility test

Hello! We are working on releasing bincode 2, and we want to make sure that bincode 2 produces the same output as bincode 1.3.

For this, we need your help. Please read on if you're using bincode in one of your projects.

We have created a test project that can be used to test the compatibility between bincode 1 and 2. It encodes structs with a wide range of settings, checks if the outputs are the same, and then deserializes the struct and checks if the output is the same.

## Adding a test case for your project

To add a test case for your project, please follow the following steps:
- [ ] Fork https://github.com/bincode-org/bincode
- [ ] create a new file `compatibility/src/<name>.rs`.
- [ ] Add a link to your project
- [ ] Add `Licence: MIT OR Apache-2.0` if you agree to distribute your code under this license
- [ ] Add a `mod <name>;` in the `lib.rs`. Make sure it's alphabetically ordered (check the ordering in your file system).
- [ ] Add your structs.
  - Adding references to libraries is not recommended. Libraries will not be implementing `bincode 2`'s encoding/decoding system.
  - If you need references to libraries, consider adding a test case for that library, and then referencing that test.
- [ ] Make sure structs derive the following traits:
  - [ ] `serde::Serialize` and `serde::Deserialize`, like normal
  - [ ] `bincode_2::Encode` and `bincode_2::Decode`, for the bincode 2 encode/decode mechanic
  - [ ] Because the crate is renamed to `bincode_2`, you also need to add `#[bincode(crate = "bincode_2")]`
  - [ ] `Debug, PartialEq`

```rs
#[derive(Serialize, Deserialize, bincode_2::Encode, bincode_2::Decode, Debug, PartialEq)]
#[bincode(crate = "bincode_2")]
pub struct YourStruct {
}
```

- [ ] Use `rand` to be able to generate a random test case for your project.
  - [ ] For strings there is a helper function in `crate`: `gen_string(rng: &mut impl Rng) -> String`
- [ ] Add the following code:

```rs
#[test]
pub fn test() {
    let mut rng = rand::thread_rng();
    for _ in 0..1000 {
        crate::test_same(<your_rand_function>(&mut rng));
    }
}
```

For examples, see the existing cases in `compatibility/src/`.

- [ ] Open a [pull request](https://github.com/bincode-org/bincode/pulls) with the title `Bincode 1 compatibility: <name of your project>`
