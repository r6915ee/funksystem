<div align="center">
  <h1><img src="assets/logo.png" alt="'FunkSystem' Logo"></h1>
</div>

> [!NOTE]
> The following lists available mirrors of the repository:
> * [Codeberg](https://codeberg.org/r6915ee/funksystem) (Official repository,
> contribute by sending issues and pull requests here!)
> * [GitHub](https://github.com/r6915ee/funksystem)

> [!CAUTION]
> FunkSystem is still in the early stages of development. Contributions are
> welcome under the MIT license.
>
> [Codeberg projects](https://codeberg.org/r6915ee/funksystem/projects) is used
> to detail what needs to be done for a release.

**FunkSystem** is a fangame engine for
[Friday Night Funkin'](https://funkin.me/), a well-known rhythm game, that aims
to primarily be data-oriented; that is, FunkSystem introduces the following
differences from most other fangame engines for Friday Night Funkin':

* FunkSystem uses the **Entity Component System** paradigm, short for **ECS**.
  Entities comprise of components, components define data, and systems operate
  on entities with certain components. This allows the game to rely on
  composition primarily instead of inheritance, allowing easier modification
  for most things.
* Unlike most other engines, FunkSystem is made with
  [Rust](https://rust-lang.org/), which allows it to make use of the advantages
  of the ecosystem. Alongside this, FunkSystem uses the
  [Bevy](https://bevy.org/) engine internally.
* FunkSystem is designed to rely on a client architecture by being modular.
  This allows multiple clients to be built, allowing custom clients to be
  built more properly.
* Most prior concepts, such as Sparrow Atlases and JSON, are discarded to aid
  with the client architecture. However, utilities are available for migrating
  certain assets.

## Building

> [!TIP]
> [NixOS](https://nixos.org/) users have a Nix shell configuration available
> that introduces most of the dependencies.

The [Rust toolchain](https://rust-lang.org/) is necessary for compilation.
Either use Rustup, or install the compiler, Cargo, and Clippy.

Additionally, albeit not required, [`just`](https://github.com/casey/just) has
many recipes that are shorthands for a variety of tasks; for example,
documentation tests happen before the documentation actually gets generated in
the `doc` recipe. For this reason, the below examples will use `just`.

Compilation can be performed using the typical subcommand:

```sh
# Builds the workspace using the "dev" profile.
just dev
```

This will take a noticeably long amount of time initially. However, due to the
way that Cargo works with the Rust compiler, further compilations should be
faster, partly due to the way that Bevy is set up.

### Documentation

Documentation can be tested and then generated using the following recipe:

```sh
just doc
```

Additionally, a `view-docs` recipe exists that allows triggering an HTTP
server defined using `$HTTP_SERVER`. This opens the specified HTTP server in
the `target/doc` directory. The following examples assumes that
[http-server](https://github.com/http-party/http-server) is the server to use:

```sh
export HTTP_SERVER=http-server
just view-docs
```

## License

The project uses the [MIT license](LICENSE).
