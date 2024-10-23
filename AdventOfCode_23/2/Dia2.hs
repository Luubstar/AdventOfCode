module Dia2 where
 import Data.List.Split(splitOn)
 import Debug.Trace (trace)
 ------------PARTE 1 -------------------
 type Set = (Int, Int, Int)

 part1 :: [String] -> Int 
 part1 (x:xs) = (isPossible x) + part1 xs
 part1 [] = 0

 isPossible :: String -> Int 
 isPossible x = do
  let set = listOfSets (splitOn ";" (unwords (drop 2 (words x))))
  splitID (words x) (isLegal set)

 splitID :: [String] -> Bool -> Int 
 splitID x b
  | b = read $ init $ x!!1
  | otherwise = 0

 listOfSets :: [String] -> [Set]
 listOfSets (x:xs) = (getBlocks (words x) (0,0,0)) : listOfSets xs
 listOfSets [] = []

 isLegal :: [Set] -> Bool
 isLegal ((c1,c2,c3):xs) = (c1 <= 12 && c2 <= 13 && c3 <= 14) && isLegal xs
 isLegal [] = True

 getBlocks :: [String] -> Set -> Set
 getBlocks (x:y:xs) (c1, c2, c3)
  | init y == "blue" || y == "blue" = getBlocks xs (c1, c2, c3 + (read x))  
  | init y == "red" || y == "red" = getBlocks xs (c1 + (read x), c2, c3)
  | init y == "green" || y == "green" = getBlocks xs (c1, c2 + (read x), c3)
 getBlocks [] s = s
 getBlocks [x] s = error x 

 ------- PARTE 2 -------
 part2 :: [String] -> Int
 part2 (x:xs) =  (getPower x) + part2(xs)
 part2 [] = 0

 getPower :: String -> Int 
 getPower x = do
  let set = listOfSets (splitOn ";" (unwords (drop 2 (words x))))
  potencia (getMax set (0,0,0)) 

 getMax :: [Set] -> Set -> Set
 getMax ((c1,c2,c3):xs) (x,y,z) = getMax xs ((max c1 x), (max c2 y), (max c3 z))
 getMax [] s = s

 potencia :: Set -> Int
 potencia (x,y,z) = x*y*z

 main = do
  contents <- (readFile "Input1.txt")
  let lin = lines contents
  putStrLn $ show $ part1 lin
  putStrLn "--------"
  putStrLn $ show $ part2 lin
