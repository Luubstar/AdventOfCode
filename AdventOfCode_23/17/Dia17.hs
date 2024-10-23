module Dia17 where 

-- ##############################################
-- ## Imports                                  ##
-- ##############################################

  import Data.Char (digitToInt)

-- ##############################################
-- ## Definiciones de datos, tipos y clases    ##
-- ##############################################  

  data Distance a = Dist a | Infinity deriving (Show, Eq)
  type Node = (Int, Int)  -- Coordenadas (fila, columna)
  type Weight = Int
  type Edge = (Node, Node, Weight)  -- (Inicio, Fin, Costo)
  type Graph = [Edge]

 -- ##############################################
-- ## Función main                             ##
-- ##############################################

  main = do
   contents <- (readFile "Input17.txt")
   let lin = map toInt $ lines contents
       graph = matrixToGraph lin 

   putStrLn $ show $ matrixToGraph lin
   --putStrLn $ show $ part1 lin
   putStrLn "--------"
   --putStrLn $ show $ part2 lin 
  
-- ##############################################
-- ## Funciones de la 1º parte                 ##
-- ##############################################


-- ##############################################
-- ## Funciones de la 2º parte                 ##
-- ##############################################



-- ##############################################
-- ## Funciones generales                      ##
-- ##############################################


  toInt :: String -> [Int] 
  toInt l = map digitToInt l   

  matrixToGraph :: [[Int]] -> Graph
  matrixToGraph matrix = concatMap createEdges indexedNodes
    where
      numRows = length matrix
      numCols = length (head matrix)
      indexedNodes = [((i, j), matrix !! i !! j) | i <- [0..numRows-1], j <- [0..numCols-1]]

      createEdges :: (Node, Weight) -> [Edge]
      createEdges ((i, j), cost) =
        let neighbors = filter isValidNeighbor [(i-1, j), (i+1, j), (i, j-1), (i, j+1)]
            isValidNeighbor (row, col) = row >= 0 && row < numRows && col >= 0 && col < numCols
            neighborEdges = map (\(row, col) -> ((i, j), (row, col), matrix !! row !! col)) neighbors
        in neighborEdges
