(def test (fn (x) (if x true false)))

(def a (test true))
(def b (test false))

(println a)
(println b)

(println [{:a a}, {:b b}])

(println ((get-kind-data GcAllocator :collect) gc_allocator))