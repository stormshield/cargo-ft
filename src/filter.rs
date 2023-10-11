use crate::package::FtPackage;

/// Partition packages into those supported and those unsupported according to
/// the package metadata.
pub fn partition_packages<'p>(
    packages: &'p [FtPackage<'p>],
    target: &str,
) -> (Vec<&'p FtPackage<'p>>, Vec<&'p FtPackage<'p>>) {
    packages.iter().partition(|p| {
        let mut targets = p.metadata().targets();

        targets.len() == 0 || targets.any(|t| t == target)
    })
}
