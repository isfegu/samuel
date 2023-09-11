# Java Demo

This is a demo to learn how to use `dothyphen_java` from a Java code.

This demo uses Java and Maven to compile the source code. Therefore, if you want to run this demo you __must__ install both.

> This demo has been tested only in GNU/Linux.

## Usage

### 1. Dynamic libray

Compile `dothyphen_java` dynamic library running from the root of the member:

```bash
dothyphen-java~ cargo build
```

The static library compiled should be placed in `target/debug/libdotjyphen.so`.

Add the (absolute) path where the library is placed to the LD_LIBRARY_PATH environment variable:

```bash
~ export LD_LIBRARY_PATH=~/Proyectos/samuel/target/debug/
```

Check that the environment variable has been set correctly:

```bash
~ echo $LD_LIBRARY_PATH
```

### 2. Java package

> To run this step is mandatory to have LD_LIBRARY_PATH set correctly.

Compile `dothyphen_java` Java package running from `dothyphen-java/src/java`:

```bash
dothyphen-java/src/java~ mvn compile
```

Generate `dothyphen_java` Java package running from `dothyphen-java/src/java`:

```bash
dothyphen-java/src/java~ mvn package
```

Install `dothyphen_java` Java package running from `dothyphen-java/src/java`:

```bash
dothyphen-java/src/java~ mvn install
```

### 3. Executing the demo

Compile the demo package running from the root of the demo directory:

```bash
demo/java~ mvn compile
```

Generate the demo package running from the root of the demo directory:

```bash
demo/java~ mvn package
```

Execute the demo running from the root of the demo directory:

```bash
demo/java~ java -jar target/demo-0.1.0.jar
```
