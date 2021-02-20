(def test (fn [x] (if x true false)))

(def a (test true))
(def b (test (bool.not true)))

(println a, (kind.of a))
(println b, (kind.of a))
(println (= (kind.of a), (kind.of b)))

(println [{:a a}, {:b b}])

(println "cleaned" (gc_allocator.collect) "bytes")