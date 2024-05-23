module Dia13 where 
-- ##############################################
-- ## Imports                                  ##
-- ##############################################

  import Data.List (transpose, groupBy)

-- ##############################################
-- ## Función main                             ##
-- ##############################################

  main = do
   contents <- (readFile "Input13.txt")
   let lin = removeBlankRows $ lines contents

   putStrLn $ show $ sum $ map part1 lin 
   putStrLn "--------"
   putStrLn $ show $ sum $ map part2 lin 

-- ##############################################
-- ## Funciones de la 1º parte                 ##
-- ##############################################

  part1 :: [String] -> Int
  part1 l = (inRow (Left (transpose l))) + (100*(inRow (Left l))) 

  inRow :: Either [String] ([String], Int) -> Int
  inRow (Left l) = inRow (Right (l, 0))
  inRow (Right (l, x)) 
   | (x+1) >= length l = 0
   | l!!x == l!!(x+1) =  if checkMirror x 1 l then x+1 else inRow (Right(l,(x+1)))
   | otherwise = inRow (Right (l, (x+1)))

  checkMirror :: Int -> Int -> [String] -> Bool
  checkMirror x dx l 
   | (x+dx+1) >= length l || (x-dx) < 0 = True 
   | l!!(x-dx) == l!!(x+1+dx) = checkMirror x (dx+1) l 
   | otherwise = False

-- ##############################################
-- ## Funciones de la 2º parte                 ##
-- ##############################################

  part2 :: [String] -> Int
  part2 l =  100 * inRow'(Left (l)) + inRow'(Left (transpose l))

  inRow' :: Either [String] ([String], Int) -> Int
  inRow' (Left l) = inRow' (Right (l, 0))
  inRow' (Right (l, x)) 
   | (x+1) >= length l = 0
   | l!!x == l!!(x+1) =  if checkMirror' x 1 False l then x+1 else inRow' (Right(l,(x+1)))
   | (length $ diff (Left (l!!x, l!!(x+1)))) == 1 = if checkMirror' x 1 True l then x+1 else inRow' (Right(l,(x+1)))
   | otherwise = inRow' (Right (l, (x+1)))

  checkMirror' :: Int -> Int -> Bool -> [String] -> Bool
  checkMirror' x dx errado l 
   | (x+dx+1) >= length l || (x-dx) < 0 = errado 
   | l!!(x-dx) == l!!(x+1+dx) = checkMirror' x (dx+1) errado l
   | (length $  diff (Left (l!!(x-dx), l!!(x+1+dx)))) == 1 && (not $ errado) =  checkMirror' x (dx+1) True l
   | otherwise = False

-- ##############################################
-- ## Funciones generales                      ##
-- ##############################################

  removeBlankRows :: [String] -> [[String]]
  removeBlankRows = filter (\ c -> length c /= 1) . (groupBy (\a b -> not (null a) && not (null b)))

  diff :: Either (String, String) (String, String, Int) -> [Int]
  diff (Right ((x:xs),(y:ys),p))
   | x /= y = [p] ++ diff (Right (xs,ys,(p+1)))
   | otherwise = diff (Right (xs,ys,(p+1)))
  diff (Right ([], [], _)) = []
  diff (Left (l, s)) = diff (Right (l,s,0))


