#lang racket

(define read-file (lambda (filename)
    (car (file->lines filename))
))

(define chars (map (lambda (x) (string x)) (string->list (read-file "input"))))
(define ints (list->vector (map (lambda (x) (string->number x)) chars)))

(define (calculate-next-1 cs pos)
    (if (= (vector-length cs) (+ pos 1)) 
        (vector-ref cs 0)
        (vector-ref cs (+ pos 1))
    )
)

(define (calculate-next-2 cs pos)
    (let ([half (/ (vector-length cs) 2)])
        (if (<= half pos)
            (vector-ref cs (- pos half))
            (vector-ref cs (+ pos half))
        )
    )
)

(define (solution cs f)
    (solution_ cs (- (vector-length cs) 1) f)
)
(define (solution_ cs pos f)
    (if (= pos 0)
        0
        (+ (solution_ cs (- pos 1) f) (microsolution cs pos f))
    )
)
(define (microsolution cs pos f)
    (let ([current-value (vector-ref cs pos)])
        (if
            (= current-value (f cs pos))
            current-value
            0
        )
    )
)

(println (solution ints calculate-next-1))
(println (solution ints calculate-next-2))