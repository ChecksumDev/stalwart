/*
 * SPDX-FileCopyrightText: 2020 Stalwart Labs LLC <hello@stalw.art>
 *
 * SPDX-License-Identifier: AGPL-3.0-only OR LicenseRef-SEL
 */

use serde::Serialize;
use std::fmt::Display;
use utils::map::bitmap::{Bitmap, BitmapItem};

use crate::collection::SyncCollection;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, Serialize)]
#[repr(u8)]
pub enum DataType {
    #[serde(rename = "Email")]
    Email = 0,
    #[serde(rename = "EmailDelivery")]
    EmailDelivery = 1,
    #[serde(rename = "EmailSubmission")]
    EmailSubmission = 2,
    #[serde(rename = "Mailbox")]
    Mailbox = 3,
    #[serde(rename = "Thread")]
    Thread = 4,
    #[serde(rename = "Identity")]
    Identity = 5,
    #[serde(rename = "Core")]
    Core = 6,
    #[serde(rename = "PushSubscription")]
    PushSubscription = 7,
    #[serde(rename = "SearchSnippet")]
    SearchSnippet = 8,
    #[serde(rename = "VacationResponse")]
    VacationResponse = 9,
    #[serde(rename = "MDN")]
    Mdn = 10,
    #[serde(rename = "Quota")]
    Quota = 11,
    #[serde(rename = "SieveScript")]
    SieveScript = 12,
    #[serde(rename = "Calendar")]
    Calendar = 13,
    #[serde(rename = "CalendarEvent")]
    CalendarEvent = 14,
    #[serde(rename = "CalendarEventNotification")]
    CalendarEventNotification = 15,
    #[serde(rename = "AddressBook")]
    AddressBook = 16,
    #[serde(rename = "ContactCard")]
    ContactCard = 17,
    #[serde(rename = "FileNode")]
    FileNode = 18,
    None = 19,
}

#[derive(Debug, Clone, Copy)]
pub struct StateChange {
    pub account_id: u32,
    pub change_id: u64,
    pub types: Bitmap<DataType>,
}

impl StateChange {
    pub fn new(account_id: u32, change_id: u64) -> Self {
        Self {
            account_id,
            change_id,
            types: Default::default(),
        }
    }

    pub fn set_change(&mut self, type_state: DataType) {
        self.types.insert(type_state);
    }

    pub fn with_change(mut self, type_state: DataType) -> Self {
        self.set_change(type_state);
        self
    }

    pub fn has_changes(&self) -> bool {
        !self.types.is_empty()
    }
}

impl BitmapItem for DataType {
    fn max() -> u64 {
        DataType::None as u64
    }

    fn is_valid(&self) -> bool {
        !matches!(self, DataType::None)
    }
}

impl From<u64> for DataType {
    fn from(value: u64) -> Self {
        match value {
            0 => DataType::Email,
            1 => DataType::EmailDelivery,
            2 => DataType::EmailSubmission,
            3 => DataType::Mailbox,
            4 => DataType::Thread,
            5 => DataType::Identity,
            6 => DataType::Core,
            7 => DataType::PushSubscription,
            8 => DataType::SearchSnippet,
            9 => DataType::VacationResponse,
            10 => DataType::Mdn,
            11 => DataType::Quota,
            12 => DataType::SieveScript,
            13 => DataType::Calendar,
            14 => DataType::CalendarEvent,
            15 => DataType::CalendarEventNotification,
            16 => DataType::AddressBook,
            17 => DataType::ContactCard,
            18 => DataType::FileNode,
            _ => {
                debug_assert!(false, "Invalid type_state value: {}", value);
                DataType::None
            }
        }
    }
}

impl From<DataType> for u64 {
    fn from(type_state: DataType) -> u64 {
        type_state as u64
    }
}

impl TryFrom<&str> for DataType {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut hash = 0;
        let mut shift = 0;

        for &ch in value.as_bytes() {
            if shift < 128 {
                hash |= (ch as u128) << shift;
                shift += 8;
            } else {
                return Err(());
            }
        }

        match hash {
            0x006c_6961_6d45 => Ok(DataType::Email),
            0x0079_7265_7669_6c65_446c_6961_6d45 => Ok(DataType::EmailDelivery),
            0x006e_6f69_7373_696d_6275_536c_6961_6d45 => Ok(DataType::EmailSubmission),
            0x0078_6f62_6c69_614d => Ok(DataType::Mailbox),
            0x6461_6572_6854 => Ok(DataType::Thread),
            0x7974_6974_6e65_6449 => Ok(DataType::Identity),
            0x6572_6f43 => Ok(DataType::Core),
            0x6e6f_6974_7069_7263_7362_7553_6873_7550 => Ok(DataType::PushSubscription),
            0x0074_6570_7069_6e53_6863_7261_6553 => Ok(DataType::SearchSnippet),
            0x6573_6e6f_7073_6552_6e6f_6974_6163_6156 => Ok(DataType::VacationResponse),
            0x004e_444d => Ok(DataType::Mdn),
            0x0061_746f_7551 => Ok(DataType::Quota),
            0x0074_7069_7263_5365_7665_6953 => Ok(DataType::SieveScript),
            _ => Err(()),
        }
    }
}

impl DataType {
    pub fn try_from_sync(value: SyncCollection, is_container: bool) -> Option<Self> {
        match (value, is_container) {
            (SyncCollection::Email, false) => DataType::Email.into(),
            (SyncCollection::Email, true) => DataType::Mailbox.into(),
            (SyncCollection::Thread, _) => DataType::Thread.into(),
            (SyncCollection::Calendar, true) => DataType::Calendar.into(),
            (SyncCollection::Calendar, false) => DataType::CalendarEvent.into(),
            (SyncCollection::AddressBook, true) => DataType::AddressBook.into(),
            (SyncCollection::AddressBook, false) => DataType::ContactCard.into(),
            (SyncCollection::FileNode, _) => DataType::FileNode.into(),
            (SyncCollection::Identity, _) => DataType::Identity.into(),
            (SyncCollection::EmailSubmission, _) => DataType::EmailSubmission.into(),
            (SyncCollection::SieveScript, _) => DataType::SieveScript.into(),
            _ => None,
        }
    }
}

impl DataType {
    pub fn as_str(&self) -> &'static str {
        match self {
            DataType::Email => "Email",
            DataType::EmailDelivery => "EmailDelivery",
            DataType::EmailSubmission => "EmailSubmission",
            DataType::Mailbox => "Mailbox",
            DataType::Thread => "Thread",
            DataType::Identity => "Identity",
            DataType::Core => "Core",
            DataType::PushSubscription => "PushSubscription",
            DataType::SearchSnippet => "SearchSnippet",
            DataType::VacationResponse => "VacationResponse",
            DataType::Mdn => "MDN",
            DataType::Quota => "Quota",
            DataType::SieveScript => "SieveScript",
            DataType::Calendar => "Calendar",
            DataType::CalendarEvent => "CalendarEvent",
            DataType::CalendarEventNotification => "CalendarEventNotification",
            DataType::AddressBook => "AddressBook",
            DataType::ContactCard => "ContactCard",
            DataType::FileNode => "FileNode",
            DataType::None => "",
        }
    }
}

impl Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for DataType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        DataType::try_from(<&str>::deserialize(deserializer)?)
            .map_err(|_| serde::de::Error::custom("invalid JMAP data type"))
    }
}
