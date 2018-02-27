# How logical systems are made

![How logical systems are made](./how-logical-systems-are-made.svg)

This diagram was generated using [svgbobrus](https://github.com/ivanceras/svgbobrus) using the following source:

```

   HOW  LOGICAL SYSTEMS  ARE MADE
   by Sven Nilsen 2018

       .-------.   .------.   .-------.
       | start |   | end  |   | error |
       '--o----'   '--.---'   '--.----'
          |           ^          ^
          |           |          |
          v           |          |
  .-------'------.done|          |
  | create new X o----'          |
  '.------o------'               |
   ^      |ok                    |
   |      |                      |
   |yes   v                      |
  .o------'--------.             |
  | can I prove X? |             |
  '.------o--------'             |
   ^      |no                    |
   |      |                      |
   |ok    v            X already |
  .o------'----------. added     |
  | add assumption X o-----------'
  '------------------'
```
