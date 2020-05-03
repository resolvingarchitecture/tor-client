<div align="center">
  <img src="https://resolvingarchitecture.io/images/ra.png"  />

  <h1>Resolving Architecture</h1>

  <p>
    <strong>Clarity in Design</strong>
  </p>
  
  <h2>TOR Client</h2>
  
  <p>
   A client for a local TOR instance. Can be ran within the <a target="_blank" href="https://github.com/resolvingarchitecture/service-bus">Service Bus</a> as a Service.
   </p>
  
  <p>
    <a href="https://travis-ci.com/resolvingarchitecture/tor-client"><img alt="build" src="https://img.shields.io/travis/resolvingarchitecture/tor-client"/></a>
    <a href="https://crates.io/crates/tor-client"><img alt="Crate Info" src="https://img.shields.io/crates/v/tor-client.svg"/></a>
    <a href="https://docs.rs/crate/tor_client/"><img alt="API Docs" src="https://img.shields.io/badge/docs.tor-client-green"/></a>
  </p>
  <p>
    <a href="https://github.com/resolvingarchitecture/tor-client/blob/master/LICENSE"><img alt="License" src="https://img.shields.io/github/license/resolvingarchitecture/tor-client"/></a>
    <a href="https://resolvingarchitecture.io/ks/publickey.brian@resolvingarchitecture.io.asc"><img alt="PGP" src="https://img.shields.io/keybase/pgp/objectorange"/></a>
  </p>
  <p>
    <img alt="commits" src="https://img.shields.io/crates/d/tor-client"/>
    <img alt="repo size" src="https://img.shields.io/github/repo-size/resolvingarchitecture/tor-client"/>
  </p>
  <p>
    <img alt="num lang" src="https://img.shields.io/github/languages/count/resolvingarchitecture/tor-client"/>
    <img alt="top lang" src="https://img.shields.io/github/languages/top/resolvingarchitecture/tor-client"/>
    <a href="https://blog.rust-lang.org/2020/03/12/Rust-1.42.html"><img alt="Rustc Version 1.42+" src="https://img.shields.io/badge/rustc-1.42+-green.svg"/></a>
  </p>

  <h4>
    <a href="https://resolvingarchitecture.io">Info</a>
    <span> | </span>
    <a href="https://docs.rs/crate/tor_client/">Docs</a>
    <span> | </span>
    <a href="https://github.com/resolvingarchitecture/tor-client/blob/master/CHANGELOG.md">Changelog</a>
  </h4>
</div>

## Donate
Request BTC/XMR/ZEC address for a donation at brian@resolvingarchitecture.io.

## Notes
!! WIP - not stable until version 1.0 !!

## Goals

- [ ] 1.0.0 - Minimal Useful Functionality<br>
    - [ ] 0.1.0 - Minimal CLI: Handshake and Session creation<br>
    - [ ] 0.2.0 - Basic I/O: Retrieve web page as a file over TOR using CLI<br>
    - [ ] 0.3.0 - Service Bus Support: [service_bus](https://crates.io/crates/service-bus) crate implementing Service trait<br>
    - [ ] 0.4.0 - TOR hidden services: setup hidden service automatically<br>
    - [ ] 0.5.0 - Send/Receive messages using TOR hidden services<br>
    - [ ] 0.6.0 - Router control<br>
    - [ ] 0.7.0 - Test Suite<br>
    - [ ] 0.8.0 - Example CLI use cases<br>
    - [ ] 0.9.0 - Example Service use cases<br>
    - [ ] 0.10.0 - Determine if local I2P router installed<br>
    - [ ] 0.11.0 - Determine local I2P router status<br>
    - [ ] 0.12.0 - Auto-install I2P router<br>
    - [ ] 0.13.0 - README.md completed<br>
    - [ ] 0.14.0 - All code documented<br>
    - [ ] 0.15.0 - All examples documented<br>

## Setup - Ubuntu 18.04
1. Download & Install TOR Router
    ```shell script
    sudo apt install tor
    ```
2. Uncomment ControlPort 9051 in file /etc/tor/torrc
3. Start TOR
   ```shell script
    sudo systemctl start tor
    ```
4. Verify running
   ```shell script
    sudo systemctl status tor
    ```
5. Verify ports 9050 and 9051 open:
    ```shell script
    ss -nlt
    ```
6. Install libssl-dev if necessary
    ```shell script
    sudo apt-get install libssl-dev
    ```