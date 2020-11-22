(def atom.update (fn atom.update [atom, func]
  (atom.set atom (func (atom.get atom)))
))

(def counter (atom.new 0_isize))

(def counter-inc (fn counter-inc []
  (atom.update counter (fn [count] (isize.add count 1)))
))

(def counter-dec (fn counter-inc []
  (atom.update counter (fn [count] (isize.sub count 1)))
))

(println counter)

(counter-inc)
(counter-inc)
(counter-inc)

(println counter)

(counter-dec)
(counter-dec)

(println counter)