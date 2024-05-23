# Gaben

Gaben, formerly known as `gaben.exe`, is bait cheat software for [Counter-Strike 2](https://store.steampowered.com/app/730/CounterStrike_2/) that punishes players who attempt to use third-party software that allegedly gives them advantages over other legitimate players.

This idea originated from [@ScriptKid](https://www.youtube.com/@ScriptKid).

This software:

- Manipulates the original game's memory using [ReadPorcessMemory](https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-readprocessmemory) to read data from the game's process in real-time.
- Uses [WriteProcessMemory](https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-writeprocessmemory) to overwrite game memory, disrupting the cheating experience for malicious users.
- [inputbot](https://github.com/obv-mikhail/InputBot) to simulate both mouse and keyboard inputs.

## License

This project is licensed under the [MIT License](LICENSE).

## Disclaimer

This software is intended for educational and ethical purposes only. The authors do not condone or support cheating in online games.

## Credits

- [@ScriptKid](https://www.youtube.com/@ScriptKid) - Original inspiration
- [me lol](https://www.youtube.com/@theunrealtarik) - Developer

## Support

If you encounter any issues or have questions, feel free to contact me on:
- [Twitter](https://twitter.com/txreqb2w)
- [Discord](https://discord.gg/7AxxXfe5)
