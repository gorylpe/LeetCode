ss = (list[int], list[int], list[int])


class Solution:
    def findMedianSortedArrays(self, nums1: list[int], nums2: list[int]) -> float:
        print("CALCULATING")
        l1 = 0
        l2 = 0
        r1 = len(nums1) - 1
        r2 = len(nums2) - 1
        m = (len(nums1) + len(nums2) - 1) // 2
        
        # rozlaczne zbiory
        if len(nums1) != 0 and len(nums2) != 0:
            numsn = None
            if nums1[r1] < nums2[l2]:
                numsn = nums1 + nums2
            elif nums2[r2] < nums1[l1]:
                numsn = nums2 + nums1
            if numsn is not None:
                if len(numsn) % 2 == 1:
                    return numsn[m]
                else:
                    return (numsn[m] + numsn[m+1]) / 2.0
        # jeden pusty
        if len(nums1) == 0 or len(nums2) == 0:
            numsn = nums1 + nums2
            if len(numsn) % 2 == 1:
                return numsn[m]
            else:
                return (numsn[m] + numsn[m+1]) / 2.0
            
        while True:
            #ROZLACZNE ZBIORY
            print(nums1[l1:r1 + 1], nums2[l2:r2 + 1])
            if r2 != l2 and nums2[r2] > nums1[r1]:
                print("Splitting 2")
                r2b = r2
                while True:
                    print(nums2[l2:r2b + 1])
                    l2a, r2a, l2b, r2b = self.split(l2, r2b)
                    
                    discarded_element_index = l1 + r2a
                    last_elem_index = r1 + 1 + r2a + 1 - 1
                    if last_elem_index < m or (nums2[r2a] < nums1[l1] and discarded_element_index < m) :
                        l2 = l2b
                        if l2 == r2b:
                            r2 = r2b
                            break
                    else:
                        r2 = r2a
                        break
            elif r1 != l1 and nums2[r2] <= nums1[r1]:
                print("Splitting 1")
                r1b = r1
                while True:
                    print(nums1[l1:r1b + 1])
                    l1a, r1a, l1b, r1b = self.split(l1, r1b)

                    discarded_element_index = l2 + r1a
                    last_elem_index = r1a + 1 + r2 + 1 - 1
                    if last_elem_index < m or (nums1[r1a] < nums2[l2] and discarded_element_index < m):
                        l1 = l1b
                        if l1 == r1b:
                            r1 = r1b
                            break
                    else:
                        r1 = r1a
                        break
            else:
                print("Finish")
                print(nums1[l1:r1 + 1], nums2[l2:r2 + 1])
                ln = l1 + l2
                mn = m - ln
                print(l1, l2, m, mn)

                numsn = []

                if nums1[r1] < nums2[l2]:
                    if r1 + 1 <= len(nums1) - 1:
                        next1 = nums1[r1 + 1]
                        if next1 <= nums2[l2]:
                            r1 += 1
                    numsn += nums1[l1:r1 + 1] + nums2[l2:r2 + 1]
                else:
                    if r2 + 1 <= len(nums2) - 1:
                        next2 = nums2[r2 + 1]
                        if next2 <= nums1[l1]:
                            r2 += 1
                    numsn += nums2[l2:r2 + 1] + nums1[l1:r1 + 1]

                next1 = next2 = None
                if r1 + 1 <= len(nums1) - 1:
                    next1 = nums1[r1 + 1]
                if r2 + 1 <= len(nums2) - 1:
                    next2 = nums2[r2 + 1]
                if next1 is not None and next2 is not None:
                    if next1 < next2:
                        numsn.append(next1)
                    else:
                        numsn.append(next2)
                elif next1 is not None:
                    numsn.append(next1)
                elif next2 is not None:
                    numsn.append(next2)

                print(numsn)

                if (len(nums1) + len(nums2)) % 2 == 1:
                    return numsn[mn]
                else:
                    return (numsn[mn] + numsn[mn + 1]) / 2.0

    def split(self, l, r) -> (int, int, int, int):
        s = (r - l) // 2
        return l, l + s, l + s + 1, r


if __name__ == '__main__':
    # print(Solution.findMedianSortedArrays(Solution(), [], [1]))
    # print(Solution.findMedianSortedArrays(Solution(), [], [2, 3, 4]))
    # print(Solution.findMedianSortedArrays(Solution(), [], [2, 3, 4, 5]))
    # print(Solution.findMedianSortedArrays(Solution(), [1, 2, 3, 5], [4, 6, 7, 8, 9]))
    # print(Solution.findMedianSortedArrays(Solution(), [2, 2, 3, 5], [1, 6, 7, 8, 9]))
    # print(Solution.findMedianSortedArrays(Solution(), [2, 2, 4, 4], [2, 2, 2, 4, 4]))
    # print(Solution.findMedianSortedArrays(Solution(), [2, 2, 4, 4], [2, 2, 4, 4, 4]))
    # print(Solution.findMedianSortedArrays(Solution(), [-1, 0, 1, 2], [3, 4]))
    # print(Solution.findMedianSortedArrays(Solution(), [1, 2], [3, 4]))
    # print(Solution.findMedianSortedArrays(Solution(), [4], [1, 2, 3, 5]))
    # print(Solution.findMedianSortedArrays(Solution(), [4,5,6], [1, 2, 3, 7]))
    print(Solution.findMedianSortedArrays(Solution(), [1,3,4,7], [2,5,6,8,9]))

    # assert Solution.findMedianSortedArrays(Solution(), [], [1]) == 1.0
    # assert Solution.findMedianSortedArrays(Solution(), [], [2, 3]) == 2.5
    # assert Solution.findMedianSortedArrays(Solution(), [1, 3], [2]) == 2.0
    # assert Solution.findMedianSortedArrays(Solution(), [1, 2], [3, 4]) == 2.5
    # assert Solution.findMedianSortedArrays(Solution(), [1], [2, 3, 4, 5, 6, 7, 8, 9]) == 5.0
    # assert Solution.findMedianSortedArrays(Solution(), [2, 2, 4, 4], [2, 2, 2, 4, 4]) == 2.0
    # assert Solution.findMedianSortedArrays(Solution(), [1, 2, 2], [1, 2, 3]) == 2.0
    # assert Solution.findMedianSortedArrays(Solution(), [], [2, 3, 4]) == 3.0
    # assert Solution.findMedianSortedArrays(Solution(), [1], [2, 3, 4]) == 2.5
    # assert Solution.findMedianSortedArrays(Solution(), [1, 2, 3, 5], [4, 6, 7, 8, 9]) == 5.0
    # assert Solution.findMedianSortedArrays(Solution(), [2, 2, 3, 5], [1, 6, 7, 8, 9]) == 5.0
    # assert Solution.findMedianSortedArrays(Solution(), [4], [1, 2, 3, 5]) == 3.0
    # assert Solution.findMedianSortedArrays(Solution(), [4,5,6], [1, 2, 3, 7]) == 4.0
