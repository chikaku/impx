//! 排序算法

/// 冒泡排序
///
/// <https://oi-wiki.org/basic/bubble-sort/>  
/// <https://zh.wikipedia.org/wiki/冒泡排序>  
/// 时间复杂度: 最优 O(n) 最坏 O(n^2) 平均 O(n^2)  
/// 稳定性: 是  
///
/// 对未排序序列的每对相邻元素进行比较, 将较大者放到后面, 每次循环后序列的最后一个元素将会是最大元素  
/// 对剩下未排序的元素从头开始进行相同的操作
///
/// ```
/// #![feature(is_sorted)]
/// use impx::sorting::bubble_sort;
///
/// let mut a = [3, 1, 2, 2, -1];
/// bubble_sort(&mut a);
/// assert!(a.is_sorted());
/// ```
pub fn bubble_sort<T: PartialOrd>(v: &mut [T]) {
    let n = v.len();
    for i in 0..(n - 1) {
        for j in 0..(n - 1 - i) {
            if v[j] > v[j + 1] {
                v.swap(j, j + 1)
            }
        }
    }
}

/// 鸡尾酒排序
///
/// <https://zh.wikipedia.org/wiki/鸡尾酒排序>  
/// 时间复杂度: 最优 O(n) 最坏 O(n^2) 平均 O(n^2)  
/// 稳定性: 是
///
/// 相当于冒泡排序的变形, 冒泡排序只在一个方向上进行比较  
/// 冒泡排序在每次遍历结束后从头开始重新遍历, 鸡尾酒排序在从头到尾的遍历结束后会从尾部向头部遍历
///
/// - 最低未排序索引处开始向后对相邻数据进行两两比较, 较大的元素放到后面
/// - 到达最高未排序索将最高未排序索引 -1 然后转换方向向前两两比较, 较小的元素放到前面的位置
/// - 回到最低未排序索引后将最低未排序索引 +1 然后继续转换方向向后比较 ...
///
/// ```
/// #![feature(is_sorted)]
/// use impx::sorting::cocktail_sort;
///
/// let mut a = [3, 1, 2, 2, -1];
/// cocktail_sort(&mut a);
/// assert!(a.is_sorted());
/// ```
pub fn cocktail_sort<T: PartialOrd>(v: &mut [T]) {
    let (mut low, mut high) = (0, v.len() - 1);
    let mut swapped = true;

    while swapped {
        swapped = false;
        for i in (low + 1)..=high {
            if v[i] < v[i - 1] {
                v.swap(i, i - 1);
                swapped = true;
            }
        }
        high -= 1;

        for i in ((low + 1)..=high).rev() {
            if v[i] < v[i - 1] {
                v.swap(i, i - 1);
                swapped = true;
            }
        }
        low += 1;
    }
}

/// 选择排序
///
/// <https://oi-wiki.org/basic/selection-sort/>  
/// <https://zh.wikipedia.org/wiki/选择排序>  
/// 时间复杂度: 最优 O(n^2) 最坏 O(n^2) 平均 O(n^2)  
/// 稳定性: 否 每次遍历后未排序部分的第一个元素被交换到最后
///
/// 选择排序的思想是每次选择未排序部分的最小的元素添加到排序部分的末尾
///
/// - 从 0 开始对未排序部分进行遍历, 记录遍历过程中的最小值所在索引
/// - 遍历完成后将最小值所在索引与 0 交换
/// - 从 1 开始对未排序部分进行遍历最后把最小元素交换到 1 的位置
/// - 以此类推
///
/// ```
/// #![feature(is_sorted)]
/// use impx::sorting::selection_sort;
///
/// let mut a = ['H', 'E', 'L', 'L', 'O'];
/// selection_sort(&mut a);
/// assert!(a.is_sorted());
/// ```
pub fn selection_sort<T: PartialOrd>(v: &mut [T]) {
    let n = v.len();
    for i in 0..(n - 1) {
        let mut min_idx = i;
        for j in (i + 1)..n {
            if v[j] < v[min_idx] {
                min_idx = j;
            }
        }
        v.swap(i, min_idx);
    }
}

/// 插入排序
///
/// <https://oi-wiki.org/basic/insertion-sort/>  
/// <https://zh.wikipedia.org/wiki/插入排序>  
/// 时间复杂度: 最优 O(n) 最坏 O(n^2) 平均 O(n^2)  
/// 稳定性: 是  
///
/// 插入排序的思想是对于任意一个未排序元素, 在已排序部分中找到适当的位置插入  
/// 在寻找到合适位置的过程中, 如果某个元素 n 不符合要求, 则 n 应该交换到向后的一个位置, 用来给待插入元素挤出空间
///
/// - 从 1 开始将前面的部分视为已经排好序
/// - 对于一个未排序的元素，在已排序部分中从后往前查找
/// - 如果排序元素大于当前元素，则将其交换到后一个位置
/// - 直到找到一个不大于当前元素的位置，插入到之前挤出的空位上
///
/// ```
/// #![feature(is_sorted)]
/// use impx::sorting::insertion_sort;
///
/// let mut a = ['x', '0', '_', '0', 'x'];
/// insertion_sort(&mut a);
/// assert!(a.is_sorted());
/// ```
pub fn insertion_sort<T: PartialOrd>(v: &mut [T]) {
    let n = v.len();
    for i in 1..n {
        let mut j = i;
        while j > 0 && v[j - 1] > v[j] {
            v.swap(j, j - 1);
            j -= 1;
        }
    }
}

/// 希尔排序
///
/// <https://oi-wiki.org/basic/shell-sort/>  
/// <https://zh.wikipedia.org/wiki/希尔排序>  
/// 时间复杂度: 最优 O(n(log^2n)) 最坏 O(n^2)  
/// 稳定性: 否
///
/// 希尔排序相当于是插入排序的优化版本, 可以避免尾部的小元素经过 n 步才能交换到正确的位置  
/// 通过将数组分组, 组间比较而非相邻一一比较, 使得小元素能够快速移动到数组前部近似正确的位置  
/// 且插入排序对于近似排好序的元素十分高效, 所以即使进行了多次排序整体依然是高效的
///
/// - 希尔排序首先将数据按 N 个一组进行分组
/// - 在插入排序中从 1 开始将前面的元素视为已经排序的, 希尔排序中将前 H 个元素视为已经排序的
/// - 在插入排序中对于未排序元素一一比较, 希尔排序中只对前面一个组中的一个元素进行比较
/// - 一次插入排序执行完成后降低组大小进行下一次插入排序直至组大小为 1 退化为正常的插入排序
/// - 此时大部分数据都已经移动到了近似正确的位置
///
/// ```
/// #![feature(is_sorted)]
/// use impx::sorting::shell_sort;
///
/// let mut a = ['x', '0', '_', '0', 'x'];
/// shell_sort(&mut a);
/// assert!(a.is_sorted());
/// ```
pub fn shell_sort<T: PartialOrd>(v: &mut [T]) {
    let n = v.len();
    let mut h = 1;
    while h < n / 3 {
        h = h * 3 + 1;
    }

    while h >= 1 {
        for i in h..n {
            let mut j = i;
            while j >= h && v[j - h] > v[j] {
                v.swap(j, j - h);
                j -= h;
            }
        }
        h /= 3;
    }
}

/// 桶排序
///
/// <https://oi-wiki.org/basic/bucket-sort/>  
/// <https://zh.wikipedia.org/wiki/桶排序>  
/// 时间复杂度: 最优 O(n) 最坏 O(n^2) 平均 O(n+n^2/k+k)  
/// 空间复杂度: 需要额外的桶空间 O(k)  
/// 稳定性: 依赖于内层排序算法, 如果用插入排序则为稳定
///
/// 桶排序的思想是设置一些整体有序的桶, 通过把序列中的元素按照大小分布分散到各个桶之中  
/// 接着在每个桶内部做排序, 通常使用插入排序  
/// 最后按桶的顺序, 将桶内有序元素取出依次放入到原序列内
///
/// - 设置一个定量的数组, 数组中的每个元素相当于一个桶
/// - 遍历序列将数据放入对应的桶之中
/// - 遍历所有桶将桶内的数据排序, 可以使用插入排序, 也可以递归使用桶排序
/// - 按顺序将桶中的元素放入原先的序列
///
/// ```
/// #![feature(is_sorted)]
/// use impx::sorting::bucket_sort;
///
/// let mut a = [1, 10000, 88992, 92, 573, 8888];
/// bucket_sort(&mut a);
/// assert!(a.is_sorted());
/// ```
pub fn bucket_sort(v: &mut [usize]) {
    let (min_val, max_val) = match (v.iter().min(), v.iter().max()) {
        (Some(min), Some(max)) if min != max => (min, max),
        _ => return,
    };

    const BUCKET_SIZE: usize = 8;

    let bucket_count = (max_val - min_val) / BUCKET_SIZE + 1;
    let mut buckets = vec![vec![]; bucket_count];

    for i in 0..v.len() {
        buckets[(v[i] - min_val) / BUCKET_SIZE].push(v[i])
    }

    let mut index = 0;
    for val in &mut buckets {
        let bucket = val;
        insertion_sort(bucket);
        for &mut val in bucket {
            v[index] = val;
            index += 1;
        }
    }
}

/// 计数排序
///
/// <https://oi-wiki.org/basic/counting-sort/>  
/// <https://zh.wikipedia.org/wiki/计数排序>  
/// 时间复杂度: 最优 O(n+k) 最坏 O(n+k) 平均 O(n+k)  
/// 空间复杂度: 需要额外的计数空间 O(k)  
/// 稳定性: 是
///
/// 假设已知待排序序列的阈值大小在 [0, N) 内则可以建立一个大小为 N 的计数空间  
/// 之后遍历未排序序列, 将每个数出现的次数添加到计数空间  
/// 最后遍历计数空间按照顺序添加对应次数的数值  
/// 某些情况下如果需要知道某个具体数据在排序数组中的位置, 则可以使用求计数空间求前缀和  
/// 计数空间每一项的值就变成了这个数值前面的数值的个数加上自己本身的个数, 也就是排序后数值所在的最后一个位置  
/// 所以在赋值时要注意从最后一个开始赋值, 这样也保证了稳定性
///
/// ```
/// #![feature(is_sorted)]
/// use impx::sorting::countint_sort;
///
/// let mut a = [1, 6666, 92, 573, 8888];
/// countint_sort(&mut a, 10000);
/// assert!(a.is_sorted());
/// ```
pub fn countint_sort(v: &mut [usize], up_bound: usize) {
    let mut counter = vec![0; up_bound];
    for i in 0..v.len() {
        counter[v[i]] += 1;
    }

    let mut index = 0;
    for (val, n) in counter.iter().enumerate() {
        for _ in 0..*n {
            v[index] = val;
            index += 1;
        }
    }
}

/// 基数排序
///
/// <https://oi-wiki.org/basic/radix-sort/>  
/// <https://zh.wikipedia.org/基数排序>  
/// 时间复杂度: 最优 O(n*k+w) 最坏 O(n*k)  
/// 空间复杂度: 内层如果使用计数排序需要额外 O(k) 的空间  
/// 稳定性: 是
///
/// 如果待排序的数据可以分为 k 个部分  
/// 比如 10 进制整数 100,102 都可以分为 3 位, 字符串 "Hello" 可以分为 5 个字符  
/// 此处的位(部分)可以按照数据的特征自定义, 只要能够进行比较  
///
/// 在进行排序时, 可以先按照最低位 k 进行比较排序, 再往上按照第 k-1 位进行比较排序  
/// 在每个位上的排序需要是稳定的, 因为要保证按照高位排序后原本低位上的顺序不能够改变    
/// 这样最终比较完最高位后整体是有序排列的  
/// 每个位上的值域 w 一般都很小可以使用计数排序
///
/// ```
/// #![feature(is_sorted)]
/// use impx::sorting::radix_sort;
///
/// let mut a = [329, 457, 657, 839, 436, 720, 355];
/// radix_sort(&mut a);
/// assert!(a.is_sorted());
/// ```
pub fn radix_sort(v: &mut [usize]) {
    let max_val = match v.iter().max() {
        Some(&val) => val,
        None => return,
    };

    let mut k = 1;
    while max_val / k > 0 {
        // 这里以 10 为步长以 10 进制方式处理数字
        // 同理也可以使用 2 进行处理
        const SIZE: usize = 10;
        let mut counter = vec![0; SIZE];
        for i in 0..v.len() {
            counter[(v[i] / k) % SIZE] += 1;
        }

        for i in 1..SIZE {
            // 记录 i 之前的元素个数
            counter[i] += counter[i - 1];
        }

        // 这里是逆序的因为 counter 保存的是排在自己前面的元素个数加上自己的个数
        // 也就是自己的最后一个索引, 所以要保证顺序, 从最后一个开始往前赋值
        let v_clone = v.to_owned();
        for i in (0..v_clone.len()).rev() {
            counter[(v_clone[i] / k) % SIZE] -= 1;
            v[counter[(v_clone[i] / k) % SIZE]] = v_clone[i];
        }

        k *= 10;
    }
}

/// 归并排序
///
/// <https://oi-wiki.org/basic/merge-sort/>  
/// <https://zh.wikipedia.org/wiki/归并排序>  
/// 时间复杂度: 最优 O(nlogn) 最坏 O(nlogn) 平均 O(nlogn)  
/// 空间复杂度: 由于合并的需求需要 O(n) 的额外空间  
/// 稳定性: 是
///
/// 分治思想, 先将未排序序列分为两部分, 对两部分各自做排序  
/// 然后从两个已排序序列的第一个元素开始进行比较, 取较小的元素
///
/// ```
/// #![feature(is_sorted)]
/// use impx::sorting::merge_sort;
///
/// let mut a = [329, 457, 657, 839, 436, 720, 355];
/// merge_sort(&mut a);
/// assert!(a.is_sorted());
/// ```
pub fn merge_sort<T: PartialOrd + Copy>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }

    let mid = v.len() / 2;
    let mut a = (v[..mid]).to_owned();
    let mut b = (v[mid..]).to_owned();

    merge_sort(&mut a);
    merge_sort(&mut b);

    let (mut i, mut j) = (0, 0);
    while i < a.len() || j < b.len() {
        if i >= a.len() {
            v[i + j] = b[j];
            j += 1;
            continue;
        }

        if j >= b.len() {
            v[i + j] = a[i];
            i += 1;
            continue;
        }

        if a[i] < b[j] {
            v[i + j] = a[i];
            i += 1;
        } else {
            v[i + j] = b[j];
            j += 1;
        }
    }
}

/// 堆排序
///
/// <https://oi-wiki.org/basic/heap-sort/>  
/// <https://zh.wikipedia.org/wiki/堆排序>  
/// 时间复杂度: 最优 O(nlogn) 最坏 O(nlogn) 平均 O(nlogn)  
/// 稳定性: 否
///
/// 利用二叉堆的性质, 建立最大堆, 然后把最大堆的数据和最后一个元素交换这样就获得了最大的元素
/// 接着从根重建二叉堆(下沉)找出第二大的元素, 将其交换到倒数第二个元素, 以此类推  
/// 节点 i 的左右子节点分别是 i*2+1 和 i*2+2 节点 i 的父节点是 (i-1)/2
///
/// ```
/// #![feature(is_sorted)]
/// use impx::sorting::heap_sort;
///
/// let mut a = [329, 457, 657, 839, 436, 720, 355];
/// heap_sort(&mut a);
/// assert!(a.is_sorted());
/// ```
pub fn heap_sort<T: PartialOrd>(v: &mut [T]) {
    fn down<T: PartialOrd>(v: &mut [T], start: usize, end: usize) {
        let mut parent = start;
        let mut child = parent * 2 + 1;

        while child <= end {
            if child < end && v[child + 1] > v[child] {
                child += 1;
            }

            if v[parent] >= v[child] {
                return;
            }

            v.swap(parent, child);
            parent = child;
            child = parent * 2 + 1;
        }
    }

    // 从最后一个节点的父节点开始堆化
    for i in (0..=(v.len() - 1 - 1) / 2).rev() {
        down(v, i, v.len() - 1);
    }

    // 每次将堆顶节点交换出来
    for i in (1..=(v.len() - 1)).rev() {
        v.swap(0, i);
        down(v, 0, i - 1);
    }
}

/// 快速排序
///
/// <https://oi-wiki.org/basic/quick-sort/>  
/// <https://zh.wikipedia.org/wiki/快速排序>  
/// 时间复杂度: 最优 O(nlogn) 最坏 O(n^2) 平均 O(nlogn)  
/// 稳定性: 否 移动了元素
///
/// 在待排序序列中取一元素作为基准值, 一般是随机取得或直接使用第一个元素  
/// 然后将大于此元素的所有元素移动到右边, 小于此元素的所有元素移动到左边  
/// 然后对两边的元素分别做排序, 最终整个序列变成有序
///
/// ```
/// #![feature(is_sorted)]
/// use impx::sorting::quick_sort;
///
/// let mut a = [329, 457, 657, 839, 436, 720, 355];
/// quick_sort(&mut a);
/// assert!(a.is_sorted());
/// ```
pub fn quick_sort<T: PartialOrd + Copy>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }

    let mut pivot = 0;
    let (mut i, mut j) = (1, v.len() - 1);

    while i <= j {
        if v[i] < v[pivot] {
            v.swap(i, pivot);
            pivot = i;
            i += 1;
        } else {
            v.swap(i, j);
            j -= 1;
        }
    }

    quick_sort(&mut v[..pivot]);
    quick_sort(&mut v[pivot + 1..]);
}

/// 双调排序
pub fn bitonic_sort<T: PartialOrd>(v: &mut [T], up: bool) {
    if v.len() > 1 {
        let mid = v.len() >> 1;
        bitonic_sort(&mut v[..mid], true);
        bitonic_sort(&mut v[mid..], false);

        bitonic_merge(v, up);
    }
}

fn bitonic_merge<T: PartialOrd>(v: &mut [T], up: bool) {
    if v.len() > 1 {
        let mid = v.len() >> 1;
        for i in 0..mid {
            if (v[i] > v[i + mid]) == up {
                v.swap(i, i + mid);
            }
        }

        bitonic_merge(&mut v[..mid], up);
        bitonic_merge(&mut v[mid..], up);
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    fn rand_slice() -> Vec<usize> {
        let mut rng = rand::thread_rng();

        let n = rng.gen_range(32..2048);
        (0..n).map(|_| rng.gen_range(0..1000)).collect()
    }

    fn do_rand_test<F>(sorter: F)
    where
        F: Fn(&mut [usize]),
    {
        for _ in 0..32 {
            let mut v = rand_slice();
            sorter(&mut v);
            if !v.is_sorted() {
                panic!("");
            }
        }
    }

    #[test]
    fn test_bubble_sort() {
        use super::bubble_sort;
        do_rand_test(bubble_sort);
    }

    #[test]
    fn test_cocktail_sort() {
        use super::cocktail_sort;
        do_rand_test(cocktail_sort);
    }

    #[test]
    fn test_selection_sort() {
        use super::selection_sort;
        do_rand_test(selection_sort);
    }

    #[test]
    fn test_insertion_sort() {
        use super::insertion_sort;
        do_rand_test(insertion_sort);
    }

    #[test]
    fn test_shell_sort() {
        use super::shell_sort;
        do_rand_test(shell_sort);
    }

    #[test]
    fn test_bucket_sort() {
        use super::bucket_sort;
        do_rand_test(bucket_sort);
    }

    #[test]
    fn test_countint_sort() {
        use super::countint_sort;

        fn countint_sort1(v: &mut [usize]) {
            countint_sort(v, 10000);
        }

        do_rand_test(countint_sort1);
    }

    #[test]
    fn test_radix_sort() {
        use super::radix_sort;
        do_rand_test(radix_sort);
    }

    #[test]
    fn test_merge_sort() {
        use super::merge_sort;
        do_rand_test(merge_sort);
    }

    #[test]
    fn test_heap_sort() {
        use super::heap_sort;
        do_rand_test(heap_sort);
    }

    #[test]
    fn test_quick_sort() {
        use super::quick_sort;
        do_rand_test(quick_sort);
    }

    #[test]
    fn test_bitonic_sort() {
        use super::bitonic_sort;

        let mut rng = rand::thread_rng();
        for _ in 0..32 {
            let mut v = (0..128)
                .map(|_| rng.gen_range(0..1000))
                .collect::<Vec<usize>>();

            bitonic_sort(&mut v, true);
            assert!(v.is_sorted());
        }
    }
}
