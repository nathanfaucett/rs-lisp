(def test (fn [x] (if x true false)))

(def a (test true))
(def b (test (bool.not true)))

(println a)
(println b)

(println [{:a a}, {:b b}])

; (println (gc_allocator.collect default_gc_allocator))