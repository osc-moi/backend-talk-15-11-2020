# Backend Talk

"There are only two kinds of languages: the ones that people complain about and the ones nobody uses"

## Introduction

What is a backend programming language?

On a piece of software there is the presentation layer(front-end) and the data access layer(back-end). Suppose a client-server model like a website, the client is the front end(website UI) and the server is the back-end(code that serves the business logic).

Some of the back-end languages are C++, C, Java, Rust, Go among others. For the purpose of this talk we are going to look into C++, Rust and Go and do comparisons.

Rust and C++ are systems languages that can write system software like compilers and operating sytems.

Go has garbage collection. If you wanted to write system software with it you'd probably need to rewrite the go runtime and even then Go uses operating systems calls internally so its really hard pull this off. However, it does write system software to a gedree at least but it's use case is mainly for application software and server programming.

When preparing for the talk I put in effort to be as unbiased as possible but these are my assessments from my interaction with them.


## Some Backstory

### C++

C++ was created as an extension of the C language. Designed to be an efficient and flexible language similar to C but also provide high level features for program organisation. Ok, breaking this down simply it referred to as 'C with classes'.

### Rust

C++ program correctness is something that' really hard to really get right. There are seriously insidious bugs that will came to haunt you during runtime(I'll go on this later for those interested). Rust was designed be a systems language that allows you to write memory safe, high-performance applications and software.

### Go

Go emerged as an alternative to C++ and Java for the app developers in the context of what Google needed for its network servers and distributed systems. The language was created to do away with the lack of pace and difficulties involved with programming for large and scalable servers and software systems.

Built for application programming, server programming and network programming.

## Now For The Good Stuff

### 1. Developer Productivity

On developer productivity were referring to the development speed of you as a programmer finishing up on a software project. Go is on the high end of the spectrum because of its simplicity and directness. In a way Go comes with the same appeal of Python on its simplicity.

On the lower end of the spectrum there is Rust and C++. Rust and C++ come with more language features and they take longer to master. You can write C++ for over a decade and never be an expert at it, because of it's many features and compiler variants. Simply put C++ is an awesome language it's just not beginner friendly. 

Their compilation speeds are also longer than that for Go. 

### 2. Expressiveness

On expressiveness were referring to the ease to which you can express a concept. Loosely translating to the number of lines of code you need to get the job done. 

Verbosity.

Let's look at an example, computing factorials:
`
    5! = 5*4*3*2*1  = 120,
    4! = 4*3*2*1    = 24
`

Go has less features compared to Rust and C++ which means it's more verbose than either.
* on generics
* metaprogramming with macros

### 3. Concurrency

What is concurrency? ...

Go has the most straight forward concurrency model with it's built in goroutines. These goroutines are green threads. You can spawn a thousand of them with no issues. C++ 20 may have coroutines but at the moment concurrency is achieved by spawning os threads.

Green threads vs non green threads [https://stackoverflow.com/questions/5713142/green-threads-vs-non-green-threads#:~:text=Green%20threads%20are%20user%20level,libraries%20rather%20than%20the%20kernel.&text=On%20a%20multi%2Dcore%20processor,green%20thread%20implementations%20normally%20cannot.]

Green threads are implementted at the application level while non green threads require kernel calls. This means that green threads can be spawned on any platform while calls to non green threads require platform specific calls e.g. windows, linux.

Green thread memory is allocated from the heap rather than having a stack created for it by the OS. This can potentially give an order of magnitude or more increase in concurrent threads

Rust and C++ natively use kernel threads but there are green thread libraries that open source. Go has native support for green threads.

Also in C++ concurrency is hard in large scalable systems.
[https://bholley.net/blog/2015/must-be-this-tall-to-write-multi-threaded-code.html]


### 3. Execution Speeds

[talk]

### 4. Memory Safety

[talk]
