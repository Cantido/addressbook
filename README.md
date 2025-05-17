# addressbook

A small, sharp tool for reading vCard files into the command line for
processing with other command-line tools like `grep`.

## Usage

Read one or many vCard files by passing their paths as arguments.

```console
$ addressbook ~/Contacts/*.vcf
┌───────┬───────────────────┐
│ Name  │ Phone             │
├───────┼───────────────────┤
│ Mom   │ +1-347-555-0100   │
│ Dad   │ +1-347-555-0101   │
└───────┴───────────────────┘
```

Run `addressbook --help` for full usage instructions.

## License

Copyright (C) 2025 Rosa Richter

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
