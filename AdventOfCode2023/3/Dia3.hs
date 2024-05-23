module Dia3 where 
 import Data.Char(isDigit, digitToInt)
 import Data.List.Split (splitOn)
 import Debug.Trace (trace)
 import Text.Read (readMaybe)

 part1 :: [String] -> Int -> Int 
 part1 x y 
  | y < length x = (toInt x y) + part1 x (y+1)
  | otherwise = 0

 toInt :: [String] -> Int -> Int
 toInt s x 
  | x == 0           = linevalue ""                       ("." ++ (s!!(x))++".")   ("." ++ (s!!(x+1))++".")
  | x == length(s)-1 = linevalue ("." ++ (s!!(x-1))++".") ("." ++ (s!!(x))++".")   ""
  | otherwise        = linevalue ("." ++ (s!!(x-1))++".") ("." ++ (s!!(x))++".")   ("." ++ (s!!(x+1))++".")

 linevalue :: String -> String -> String -> Int
 linevalue _ [y] _ = 0
 linevalue [] (y:ys) (z:zs) = linevalue (take (length (y:ys)) $ repeat '.') (y:ys) (z:zs)
 linevalue (x:xs) (y:ys) [] = linevalue (x:xs) (y:ys) (take (length (y:ys)) $ repeat '.')
 linevalue [] [] [] = 0
 linevalue (x:xs) (y:ys) (z:zs) 
  | isDigit (ys!!0) = do
     let value =  head (splitOn "." ys)
     let vlen = (length value) + 1 
     let p = ((x:  (take vlen xs)),(y:(take vlen ys)),(z: (take vlen zs)))
     if (length value) > 0 then
      let action  = case () of 
                  _ | (isNumber value) && (isLegal p) -> (abs $ read value) + (linevalue (drop (vlen-1) xs) (drop (vlen-1) ys) (drop (vlen-1) zs))
                  _ | (isNumber value) && (not $ isLegal p) -> 0 + (linevalue (drop (vlen-1) xs) (drop (vlen-1) ys) (drop (vlen-1) zs))
                  _ -> (abs $ sane value "") + (linevalue (drop (vlen-1) xs) (drop (vlen-1) ys) (drop (vlen-1) zs))
       in
       action 
     else (linevalue xs ys zs)
  | otherwise = linevalue xs ys zs

 isLegal :: (String,String,String) -> Bool
 isLegal (x,y,z) = not $ all (\x -> isDigit x || x == '.') x &&  all (\x -> isDigit x || x == '.') y &&  all (\x -> isDigit x || x == '.') z

 isNumber :: String -> Bool
 isNumber str = case reads str :: [(Double, String)] of
               [(_, "")] -> True
               _         -> False 

 sane :: String -> String -> Int 
 sane (x:xs) carry
  | isDigit x = sane xs (carry++[x])
  | otherwise = if length carry > 0 then read (carry) + sane xs "" else sane xs ""
 sane [] carry 
  | length carry > 0 = read carry
  | otherwise = 0

  -----PARTE 2-----
 part2 :: [String] -> Int -> Int
 part2 x y 
  | y < length x = (toInt' x y) + part2 x (y+1)
  | otherwise = 0

 toInt' :: [String] -> Int -> Int
 toInt' s x 
  | x == 0           = linevalue' ""                       ("." ++ (s!!(x))++".")   ("." ++ (s!!(x+1))++".") 0
  | x == length(s)-1 = linevalue' ("." ++ (s!!(x-1))++".") ("." ++ (s!!(x))++".")   ""                       0
  | otherwise        = linevalue' ("." ++ (s!!(x-1))++".") ("." ++ (s!!(x))++".")   ("." ++ (s!!(x+1))++".") 0
 
 linevalue' :: String -> String -> String -> Int -> Int 
 linevalue' [] [] [] _ = 0
 linevalue' [] y z c = linevalue' (take (length y) $ repeat '.') y z c 
 linevalue' x y [] c = linevalue' x y (take (length y) $ repeat '.') c
 linevalue' x y z c 
  | c >= length y = 0
  | y!!c == '*' = do 
     let value = trace(show $ (getValue x (c+1) 0) ++ (getValue y (c+1) 0) ++ (getValue z (c+1) 0)) (getValue x (c+1) 0) ++ (getValue y (c+1) 0) ++ (getValue z (c+1) 0)
     if length(value) == 2 then trace (show value) product value + linevalue' x y z (c+1) else linevalue' x y z (c+1)
  | otherwise = linevalue' x y z (c+1)


 getValue :: String -> Int -> Int -> [Int]
 getValue s coord counter  
  | coord < 0 || counter >= 3 = []
  | not $ isDigit (s!!coord) = getValue s (coord-1) (counter+1)
  | isDigit (s!!coord) = do 
     let val =  (getNumber s coord) 
         move = getDespl s coord 
     if (coord - 1 - move) >= 0 && (counter+1+move) < 3 then val ++ getValue s (coord-1-move) (counter+1+move) else  val

    where 
    getNumber s coord 
     | isDigit (s!!coord) && coord >0 = trace("paso" ++ show coord) getNumber s (coord-1)
     | coord < 0 = trace ( "getNumber coord is 0" ++ show coord ++ " " ++ (show $ head (splitOn "." (drop coord s)))) [read $ sane' $ head (splitOn "." s) :: Int] 
     | not $ isDigit (s!!coord) = trace ("getNumber completed " ++ show coord ++ " " ++ (show $ sane' $ head (splitOn "." (drop (coord+1) s)))) [(read $ sane' $ head (splitOn "." (drop (coord+1) s))) :: Int]
    getDespl s coord
     | coord <= 0 = 0
     | isDigit (s!!coord) = 1 + getDespl s (coord - 1)
     | otherwise = 0

 sane' :: String -> String
 sane' (x:xs) 
  | isDigit x = (x:(sane' xs))
  | otherwise = ""
 sane' [] = ""
 main = do
  contents <- (readFile "Input3.txt")
  let lin = lines contents
  --putStrLn $ show $ part1 lin 0
  putStrLn "--------"
  putStrLn $ show $ part2 lin 0




