(def counter (atom.new 0_isize))

(def counter-inc (fn counter-inc []
  (atom.update counter (fn [count] (isize.add count 1)))
))

(def counter-dec (fn counter-inc []
  (atom.update counter (fn [count] (isize.sub count 1)))
))

(println "default", counter)

(counter-inc)
(counter-inc)
(counter-inc)

(println "inc 3", counter)

(counter-dec)
(counter-dec)

(println "dec 2", counter)