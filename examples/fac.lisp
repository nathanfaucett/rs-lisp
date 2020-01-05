(println ((fn fac [x] 
    (if (= x 0)
        1
        (isize.mul x, (fac (isize.sub x 1)))
    )
), 5))