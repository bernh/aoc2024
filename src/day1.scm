(define lines (get-input-lines))
(define nums (map string->int (split-whitespace lines)))

;; alternative version using foldl
(define (split_fold lst)
  (foldl 
    (lambda (item acc)
      (let ((odd-list (first acc))
            (even-list (second acc))
            (index (third acc))
           )
        (if (even? index)
          (list odd-list (cons item even-list) (+ 1 index))
          (list (cons item odd-list) even-list (+ 1 index))
        )))
    (list '() '() 0)
    lst))

(define l1 (car (split_fold nums)))
(define l2 (cadr (split_fold nums)))

(define (sum-abs-diffs l1 l2)
  (foldl
    (lambda (index acc) (+ acc (abs (- (list-ref l1 index) (list-ref l2 index)))))
    0
    (range 0 (length l1))
  ))

(define (count lst x)
  (foldl (lambda (i acc) (if (eqv? i x) (+ 1 acc) acc)) 0 lst))

(define (similarity l1 l2)
  (foldl (lambda (v acc) (+ acc (* v (count l2 v)))) 0 l1))

(define sol1 (sum-abs-diffs (list-sort l1) (list-sort l2)))
(define sol2 (similarity l1 l2))

;; return the solutions
(list sol1 sol2)
