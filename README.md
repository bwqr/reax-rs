# **reax-rs**
### __* * Please note that reax-rs is currently in implementation stage. * *__
### __* * Both documentation and implementation are in progress. * *__

**reax-rs** aims to be a reactive rust framework intended to be a backend for a GUI application.
You are expected to implement business-logic in the reax-rs and create frontends for this implementation, like an ios application written in Swift and an android application written in Kotlin.

I believe that by implementing a reactive rust framework which can be used in supported platforms will let you:

* use native APIs that are provided by platform in frontend implementation 
* extract common business-logic into one specific platform independent library
* switch frontends much more easily. Did not like implementing with Kotlin and Swift, then switch to Flutter or some Rust GUI framework

Each item in the list has also its own benefits like using native APIs will let you adopt to changes in them much more easily.

These are the features expected from reax-rs:

* Reactive store implementation
* Compile time ffi checks
* Automatic ffi code generation
* e2e testing support from frontend implementation to backend implementation 

You can find an experimental implementation of the reax-rs in this [Reax-Demo](https://github.com/bwqr/reax-demo) repository.
**Reax-Demo** repository is an experiment which will guide us to a final architecture of reactive framework, reax-rs.

Current implementation targets to mobile platforms, ios and android.
However, ultimate goal of reax-rs is to support much more platforms than mobile ones, including webassembly and dart (Flutter).

### __Seriously, why we need this?__
I also ask myself why do we need something like this. Two seperate mobile applications which contain duplicated business logic implementation can become a cumbersome to maintain.
Any change in business logic requires adding or updating two seperate implementations. These problems can be solved by using hybrid frameworks like Flutter, Nativescript etc.
This path also has its own set of problems, like using native APIs will generally push you to use some plugins with their restrictions.
Moreover, they can be outdated and become incompatible with the new version of mobile platforms or the framework itself. I have had enough experiences with plugins and solutions are ended up hacking the framework and the plugin itself.
These are my bad experiences in mobile app development which lead me to think how we can at least eliminate some shortcomings of these development models. Hence reax-rs does not aim to eliminate all the hassles in its own development model. I also wanted to try Rust in different platforms other than web app development.

This readme describes the idea in my mind, time will show if it is a good one or bad one.
