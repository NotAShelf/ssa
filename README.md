<div align="center">
    <img src="https://deps.rs/repo/github/notashelf/ssa/status.svg" alt="https://deps.rs/repo/github/notashelf/ssa">
    <img src="https://img.shields.io/github/stars/notashelf/ssa?label=stars&color=DEA584">
    <h1>SSA</h1>
    <p align="left">
    Simple, streamlined and ✨ pretty ✨  aggregator for the security report
    generated by <code>systemd-analyze security</code>. Simply collects data
    from JSON output and pretty-prints it for your viewing pleasure.
    Optionally, you can print the results in JSON for easier CI/CD integration.
    </p>
</div>

## Features

- Simple
- Fast (One could say... blazingly fast.)
- Pretty
- Verbose

What else do you need?

## Usage

You can run SSA without any arguments, which will return a pretty-printed
version of the security analysis. Though, the main functionality - the _Crème de
la crème_ - of SSA is its ability to filter services by predicate, and print
them in JSON format if need be. Lets go over possible arguments.

- `-t, --top-n <TOP_N>` -> number of top services to display
- `-p, --predicate <PREDICATE>` -> predicate by which to filter services

- `--ok` -> only return services with the **OK** predicate
- `--medium` -> only return services with the **MEDIUM** predicate
- `--exposed` -> only return services with the **EXPOSED** predicate
- `--unsafe` -> only return services with the **UNSAFE** predicate

- `--debug` ->enable debug mode to print the raw JSON output
- `--json` ->output results in JSON format

In addition, you will be shown the average exposure (out of 10, 10 being worst)
and the average happiness (out of 5, 5 being best). In addition to displaying
the top N services for a given predicate, SSA will color the exposure level
output based on how exposed it is. Because here do things the ✨ pretty ✨ way.

### Example 1:

One case is that you would combine `--top-n` and `--predicate` to print a number
of services with the predicate you wish to filter for.

```bash
ssa --top-n 10 --predicate UNSAFE
```

This will return the **10** services marked as **UNSAFE** in the security
report. Possible predicates are:

- `OK`
- `MEDIUM`
- `EXPOSED`
- `UNSAFE`

### Example 2:

Another case is that you wish to see all **UNSAFE** (scary) services on your
system, for future hardening. In that case you can simply run

```bash
ssa --unsafe
```

This will return all unsafe services. Similarly, you can filter only services
with **OK** predicate if you wish to feel more comfortable.

```bash
ssa --unsafe
```

Better yet, lets show just the _top 3_ services with **MEDIUM** predicate...

```bash
ssa --unsafe --top-n 3
```

## Why?

Honestly, just see the next section. Long story short is that I wanted to
aggregate the results of `systemd-analyze security` for testing and NixOS VM
tests in CI.

## Contributing

[Microfetch]: https://github.com/notashelf/microfetch

For what it's worth, SSA has been created because I wanted to write a structured
bash script for parsing the output of `systemd-analyze security`. Rust came to
mind, as Serde is pretty cool and I wanted to do argument parsing - which Clap
does better than Python libs I am familiar with.

The software is very minimal. You run it, you get a bunch of lines. You might
have cooler ideas to do with aggregated data (in which case, just take a look at
the parser) or inspect the raw JSON data yourself with `--debug` passed to the
program.

If you would like to see some other features, open either an issue or a pull
request depending on your own ability to implement the changes. SSA is not
restricted by petty limitations such as my other toy project, [Microfetch], and
is always open to new features.

## License

SSA is licensed under the [MIT License](LICENSE). See the license file for more
details.
