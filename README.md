# rust-library-dynamic-static
This project helps to create static and dynamic library in rust, also shows to inter-call each other.

Steps to compile 

 ### 1. Build staticlib folder. `cargo build`
 
 ```
 yashwanthsingh@Yashwanths-MacBook-Pro staticlib1 % cargo build
     ...
    Finished dev [unoptimized + debuginfo] target(s) in 0.27s 
  ```
     
  ### 2. Build dynamiclib folder. `cargo build`
 
 ```
 yashwanthsingh@Yashwanths-MacBook-Pro dynamiclib % cargo build
     ...
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
  ```
    
   ### 3. Build dynamic_static_lib folder. `cargo build`
 
 ```
 yashwanthsingh@Yashwanths-MacBook-Pro dynamic_static_lib % cargo build
    ...
    Finished dev [unoptimized + debuginfo] target(s) in 0.53s
  ```
  
  ### 4. Build and Run the application. `cargo run`
 
 ```
yashwanthsingh@Yashwanths-MacBook-Pro application % cargo run
   Compiling application v0.1.0 (/Users/yashwanthsingh/Yash/Projects/qnx/rust-library-dynamic-static/application)
    Finished dev [unoptimized + debuginfo] target(s) in 0.61s
     Running `target/debug/application`
Running from main application
Starting App
1. Running from  dymanic library
Running dymanic library calling static libray 
res of addition 3,5 =8
3. Running from static library in application
res of addition 1,2 =3
  ```
    
  

 
 
