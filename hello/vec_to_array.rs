// 5 Agustus 2020 17:49
//
fn ubah<'a>() -> &'a [i32] {
	let a = vec![1, 2, 3];

	&a
}

fn main() {
}

/*
<?php 
// Utility functions to find minimum 
// and maximum of two elements 
function mins($x, $y) 
{ 
	if($x < $y) 
		return $x; 
	else
		return $y; 
} 
	
function maxi($a, $b) 
{ 
	if($a > $b) 
		return $a; 
	else
		return $b; 
} 
	
// Returns length of the longest 
// contiguous subarray 
function findLength(&$arr, $n) 
{ 
	$max_len = 1; // Initialize result 
	for ($i = 0; $i < $n - 1; $i++) 
	{ 
		// Initialize min and max for all 
		// subarrays starting with i 
		$mn = $arr[$i]; 
		$mx = $arr[$i]; 

		// Consider all subarrays starting 
		// with i and ending with j 
		for ($j = $i + 1; $j < $n; $j++) 
		{ 
			// Update min and max in this 
			// subarray if needed 
			$mn = mins($mn, $arr[$j]); 
			$mx = maxi($mx, $arr[$j]); 

			// If current subarray has all 
			// contiguous elements 
			if (($mx - $mn) == $j - $i) 
				$max_len = maxi($max_len, 
								$mx - $mn + 1); 
		} 
	} 
	return $max_len; // Return result 
} 

// Driver Code 
$arr = array(1, 56, 58, 57, 90, 
			92, 94, 93, 91, 45); 
$n = sizeof($arr); 
echo ("Length of the longest contiguous" . 
						" subarray is "); 
echo (findLength($arr, $n)); 
	
// This code is contributed 
// by Shivi_Aggarwal. 
?> 

*/