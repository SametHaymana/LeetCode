
import java.io.*;
 
class BinarySearch {
   
    // Returns index of x if it is present in arr[].
    int binarySearch(int arr[], int x)
    {
        int l = 0, r = arr.length - 1;
        while (l <= r) {
            int m = l + (r - l) / 2;
 
            // Check if x is present at mid
            if (arr[m] == x)
                return m;
 
            // If x greater, ignore left half
            if (arr[m] < x)
                l = m + 1;
 
            // If x is smaller, ignore right half
            else
                r = m - 1;
        }
 
        // If we reach here, then element was
        // not present
        return -1;
    }
 
}


class Solution {
    public boolean searchMatrix(int[][] matrix, int target) {
        int m = matrix.length;
        int n = matrix[0].length;

        BinarySearch bs = new BinarySearch();


        // Make a 1D array from the 2D array last element of each row
        int[] arr = new int[m];
        for(int i=0; i<m; i++){
            arr[i] = matrix[i][n-1];
        }

        // Find first largest element in the array
        for(int i=0; i<m; i++){
            if(arr[i] >= target){
                // Search in the row
                int index = bs.binarySearch(matrix[i], target);
                if(index != -1){
                    return true;
                }
            }
        }


        return false;
    }
}



public class Main{
    public static void main(String[] args){
        System.out.println("Hello World");
    }
}