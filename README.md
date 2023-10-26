# dumptruck

Two things in one repo:

1. A tool I made to spam a very specific phishing google form that was sent to every student at my University, with the express purpose of making it hard for the spammer to find the real data
2. One of my very first non-trivial Rust programs

## Quick Start

1. Create the file `src/url.in` containing the endpoint of the form on the first line.
2. Edit the `FakeData` struct and `FakeData::generate()` methods to generate appropriate data for your form
3. Update the form mapping in `main`

## Disclaimer

I do not mind this code being adapted to be used against other forms (be they Google Forms or otherwise).

However, any consequences of any kind that arise from your use of this source code, whether you modified it or not, are not mine to bear. I do not endorse spam. Please use this tool responsibly.
