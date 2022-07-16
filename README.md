# RBLXWILD RAIN JOINER

Install rust from https://rust-lang.org then after installtion run cmd in your currently directory and run cargo run easy


* Had fun working on this with friends after getting banned from doing this. Made huge profits with some simple code, ill be back soon with some more fun stuff. Follow me and star this repository. 


## How to get authentication key

Some context; So the reason we need your auth is so it can login and join the rain. This is in no sort stored anywhere other then on your own device in a config that can be changed at any time. Dont belive me? Read the source code

Run this JS code in your console on https://rblxwild.com to copy your auth key
```
var text = localStorage.getItem("authToken");
navigator.clipboard.writeText(text).then(function() {
console.log('Async: Copying to clipboard was successful!');
}, function(err) {
console.error('Async: Could not copy text: ', err);
});
```


## How to get 2captcha key

2captcha is a very cheap service for solving captchas, while I have not released my free captcha solver included with this version of this bot, this is the cheapest alternative for you. Simply sign up for https://2captcha.com and copy your api key (you may need to deposit some small funds 3$< to get started.
