# Design

The primary goal is to create and manage mod packs
- [ ] Mod packs are a collection of symlinks to mods in an installed mods cache

## Packs
- [x] Create new pack
- [x] List existing packs
- [x] Rename pack
- [ ] Switch between active packs

I think I am going to do this as subcommands of the pack command, so invocation looks something like:
```bash
mpreg pack create|list|delete|rename|switch
```

## Config
To read the config, I'm going to make a config IO file.
To edit the config, the user will interact with the configs subcmd