use crate::{utils::CidrAddress, SummaryOption};
use std::{
    fmt, iter,
    net::{IpAddr, Ipv4Addr},
};

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum FsType {
    #[default]
    Ext4,
    Xfs,
}

impl fmt::Display for FsType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            FsType::Ext4 => "ext4",
            FsType::Xfs => "XFS",
        };
        write!(f, "{s}")
    }
}

pub const FS_TYPES: &[FsType] = &[FsType::Ext4, FsType::Xfs];

#[derive(Clone, Debug)]
pub struct LvmBootdiskOptions {
    pub disk: Disk,
    pub total_size: u64,
    pub swap_size: u64,
    pub max_root_size: u64,
    pub max_data_size: u64,
    pub min_lvm_free: u64,
}

impl LvmBootdiskOptions {
    pub fn defaults_from(disk: &Disk) -> Self {
        let min_lvm_free = if disk.size > 128 * 1024 * 1024 {
            16 * 1024 * 1024
        } else {
            disk.size / 8
        };

        Self {
            disk: disk.clone(),
            total_size: disk.size,
            swap_size: 4 * 1024 * 1024, // TODO: value from installed memory
            max_root_size: 0,
            max_data_size: 0,
            min_lvm_free,
        }
    }
}

#[derive(Clone, Debug)]
pub enum AdvancedBootdiskOptions {
    Lvm(LvmBootdiskOptions),
}

impl AdvancedBootdiskOptions {
    fn selected_disks(&self) -> impl Iterator<Item = &Disk> {
        match self {
            AdvancedBootdiskOptions::Lvm(LvmBootdiskOptions { disk, .. }) => iter::once(disk),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Disk {
    pub path: String,
    pub size: u64,
}

impl fmt::Display for Disk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: Format sizes properly with `proxmox-human-byte` once merged
        // https://lists.proxmox.com/pipermail/pbs-devel/2023-May/006125.html
        write!(f, "{} ({} B)", self.path, self.size)
    }
}

#[derive(Clone, Debug)]
pub struct BootdiskOptions {
    pub disks: Vec<Disk>,
    pub fstype: FsType,
    pub advanced: AdvancedBootdiskOptions,
}

#[derive(Clone, Debug)]
pub struct TimezoneOptions {
    pub timezone: String,
    pub kb_layout: String,
}

impl Default for TimezoneOptions {
    fn default() -> Self {
        Self {
            timezone: "Europe/Vienna".to_owned(),
            kb_layout: "en_US".to_owned(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct PasswordOptions {
    pub email: String,
    pub root_password: String,
}

impl Default for PasswordOptions {
    fn default() -> Self {
        Self {
            email: "mail@example.invalid".to_owned(),
            root_password: String::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct NetworkOptions {
    pub ifname: String,
    pub fqdn: String,
    pub address: CidrAddress,
    pub gateway: IpAddr,
    pub dns_server: IpAddr,
}

impl Default for NetworkOptions {
    fn default() -> Self {
        // TODO: Retrieve automatically
        Self {
            ifname: String::new(),
            fqdn: "pve.example.invalid".to_owned(),
            // Safety: The provided mask will always be valid.
            address: CidrAddress::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0).unwrap(),
            gateway: IpAddr::V4(Ipv4Addr::UNSPECIFIED),
            dns_server: IpAddr::V4(Ipv4Addr::UNSPECIFIED),
        }
    }
}

#[derive(Clone, Debug)]
pub struct InstallerOptions {
    pub bootdisk: BootdiskOptions,
    pub timezone: TimezoneOptions,
    pub password: PasswordOptions,
    pub network: NetworkOptions,
}

impl InstallerOptions {
    pub fn to_summary(&self) -> Vec<SummaryOption> {
        vec![
            SummaryOption::new("Bootdisk filesystem", self.bootdisk.fstype.to_string()),
            SummaryOption::new(
                "Bootdisks",
                self.bootdisk
                    .advanced
                    .selected_disks()
                    .map(|d| d.path.as_str())
                    .collect::<Vec<&str>>()
                    .join(", "),
            ),
            SummaryOption::new("Timezone", &self.timezone.timezone),
            SummaryOption::new("Keyboard layout", &self.timezone.kb_layout),
            SummaryOption::new("Administator email:", &self.password.email),
            SummaryOption::new("Management interface:", &self.network.ifname),
            SummaryOption::new("Hostname:", &self.network.fqdn),
            SummaryOption::new("Host IP (CIDR):", self.network.address.to_string()),
            SummaryOption::new("Gateway", self.network.gateway.to_string()),
            SummaryOption::new("DNS:", self.network.dns_server.to_string()),
        ]
    }
}
