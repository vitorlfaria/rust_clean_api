
<!-- Improved compatibility of back to top link: See: https://github.com/othneildrew/Best-README-Template/pull/73 -->
<a name="readme-top"></a>
<!--
*** Thanks for checking out the Best-README-Template. If you have a suggestion
*** that would make this better, please fork the repo and create a pull request
*** or simply open an issue with the tag "enhancement".
*** Don't forget to give the project a star!
*** Thanks again! Now go create something AMAZING! :D
-->



<!-- PROJECT SHIELDS -->
<!--
*** I'm using markdown "reference style" links for readability.
*** Reference links are enclosed in brackets [ ] instead of parentheses ( ).
*** See the bottom of this document for the declaration of the reference variables
*** for contributors-url, forks-url, etc. This is an optional, concise syntax you may use.
*** https://www.markdownguide.org/basic-syntax/#reference-style-links
-->
[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![LinkedIn][linkedin-shield]][linkedin-url]


<!-- PROJECT LOGO -->
<br />
<div align="center">

  <h1 align="center">Rust Clean Architecture Api</h1>

  <p align="center">
    An simple project template to demonstrate a api with clean architecure in Rust
    <br />
    <a href="https://github.com/vitorlfaria/rust_clean_api/issues">Report Bug</a>
    ·
    <a href="https://github.com/vitorlfaria/rust_clean_api/issues">Request Feature</a>
  </p>
</div>



<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>



<!-- ABOUT THE PROJECT -->
## About The Project

This project was born out of a passion for exploring the intricacies of crafting APIs with Rust, delving into the clean architecture paradigm. It's not just about code; it's a journey into the heart of Rust's capabilities, all in the name of honing my skills and understanding the artistry behind clean and efficient API design.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



### Built With

* [![Rust][Rust]][Rust-url]
* [![Axum][Axum]][Axum-url]
* [![SurrealDB][SurrealDB]][SurrealDB-url]

<!-- GETTING STARTED -->
## Getting Started
### Prerequisites

If you to want the feature to run in watch mode, install `cargo-watch`
* cargo-watch
  ```sh
  cargo install cargo-watch
  ```

### Installation
1. Clone the repo
   ```sh
   git clone https://github.com/vitorlfaria/rust_clean_api.git
   ```
2. Build the project to install the dependecies
   ```sh
   cargo build
   ```
4. Run with
   ```sh
   cargo run
   ```
   Or, for more dynamic development, run in watch mode
   ```sh
   cargo watch -q -c -w src/ -x run
   ```
   `-q` quiet mode <br>
   `-c` clear the console on reload <br>
   `-w` watch all change to files under `src/` <br>
   `-x` executes `run` command

<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- ROADMAP -->
## Roadmap

- [x] Working api structure
- [x] Clean Architecure
- [x] Database
- [ ] Authentication
    - [ ] Create user
    - [ ] Authenticate with JWT

See the [open issues](https://github.com/vitorlfaria/rust_clean_api/issues) for a full list of proposed features (and known issues).

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'feat: add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE.txt` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTACT -->
## Contact

Vitor Lacerda - [@vitorlacerdaf](https://instagram.com/vitorlacerdaf) - vitorlacerdafaria7@gmail.com

Project Link: [https://github.com/vitorlfaria/rust_clean_api](https://github.com/vitorlfaria/rust_clean_api)

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- ACKNOWLEDGMENTS -->
## Acknowledgments

* [Axum](https://github.com/tokio-rs/axum)
* [SurrealDB](https://surrealdb.com/)
* [Serde](https://serde.rs/)
* [Chrono](https://docs.rs/chrono/latest/chrono/)

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/vitorlfaria/rust_clean_api.svg?style=for-the-badge
[contributors-url]: https://github.com/vitorlfaria/rust_clean_api/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/vitorlfaria/rust_clean_api.svg?style=for-the-badge
[forks-url]: https://github.com/vitorlfaria/rust_clean_api/network/members
[stars-shield]: https://img.shields.io/github/stars/vitorlfaria/rust_clean_api.svg?style=for-the-badge
[stars-url]: https://github.com/vitorlfaria/rust_clean_api/stargazers
[issues-shield]: https://img.shields.io/github/issues/vitorlfaria/rust_clean_api.svg?style=for-the-badge
[issues-url]: https://github.com/vitorlfaria/rust_clean_api/issues
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[linkedin-url]: https://linkedin.com/in/vitor-lacerda-faria
[product-screenshot]: images/screenshot.png
[Rust]: https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white
[Rust-url]: https://rust-lang.org/
[Axum]: https://img.shields.io/badge/Axum-d97d0d?style=for-the-badge
[Axum-url]: https://github.com/tokio-rs/axum
[SurrealDB]: https://img.shields.io/badge/SurrealDB-cc108d?style=for-the-badge
[SurrealDB-url]: https://surrealdb.com/
