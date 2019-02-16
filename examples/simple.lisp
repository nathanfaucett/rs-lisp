(def test (fn (x) (if x true false)))

(def a (test true))
(def b (test false))

(println a)
(println b)

[{:a a}, {:b b}]