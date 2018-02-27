# How logical systems are created

![How logical systems are created](./how-logical-systems-are-created.svg)

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
