module Dia9 where 

-- ##############################################
-- ## Imports                                  ##
-- ##############################################

  import Debug.Trace (trace)
-- ##############################################
-- ## Definiciones de datos, tipos y clases    ##
-- ############################################## 



-- ##############################################
-- ## Función main                             ##
-- ##############################################

  main = do
   contents <- (readFile "Input9.txt")
   let lin = lines contents

   putStrLn $ show $ part1 (map words $ lin)
   putStrLn "--------"
   putStrLn $ show $ part2 (map words $ lin)
  
-- ##############################################
-- ## Funciones de la 1º parte                 ##
-- ##############################################
  
  part1 :: [[String]] -> Int
  part1 (x:xs) = (last $ getValue (map read x)) + (part1 xs)
  part1 [] = 0

  getValue :: [Int] -> [Int]
  getValue x = 
     let op = iterar x 
     in
     if all (== head op) (tail op) then  [(last x) + (last op)] else x ++ [((last x) + (last $ getValue op))]


  iterar :: [Int] -> [Int]
  iterar (x:y:xs) = [(y-x)] ++ (iterar (y:xs))
  iterar [x] = []

-- ##############################################
-- ## Funciones de la 2º parte                 ##
-- ##############################################
  part2 :: [[String]] -> Int
  part2 (x:xs) = (head $ getValue' (map read x)) + (part2 xs)
  part2 [] = 0

  getValue' :: [Int] -> [Int]
  getValue' x = 
     let op = iterar' x 
     in
     if all (== head op) (tail op) then [(head x) - (head op)] ++ x else  [((head x) - (head $ getValue' op))] ++ x


  iterar' :: [Int] -> [Int]
  iterar' (x:y:xs) = [(y-x)] ++ (iterar' (y:xs))
  iterar' [x] = []


-- ##############################################
-- ## Funciones generales                      ##
-- ##############################################

