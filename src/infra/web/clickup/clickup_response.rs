use crate::domain::model::clickup::{
    ClickupTask, ClickupTasks, ClickupTimeEntries, ClickupTimeEntry,
};
use crate::infra::web::{BasicResponse, BasicResponseImpl};
use anyhow::{bail, Result};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone)]
pub(crate) struct ClickupResponse(pub BasicResponseImpl);

impl From<BasicResponseImpl> for ClickupResponse {
    fn from(from: BasicResponseImpl) -> ClickupResponse {
        ClickupResponse(from)
    }
}

pub(crate) trait ParseClickupResponse {
    fn try_to_serde_json_value(&self) -> Result<Value, ClickupError>;
    fn try_to_clickup_teams(&self) -> Result<ClickupTeamsResponseDto, ClickupError>;
    fn try_to_clickup_spaces(&self) -> Result<ClickupSpacesResponseDto, ClickupError>;
    fn try_to_clickup_folders(&self) -> Result<ClickupFoldersResponseDto, ClickupError>;
    fn try_to_clickup_lists(&self) -> Result<ClickupListsResponseDto, ClickupError>;
    fn try_to_clickup_list(&self) -> Result<ClickupListResponseDto, ClickupError>;
    fn try_to_clickup_tasks(&self) -> Result<ClickupTasksResponseDto, ClickupError>;
    fn try_to_clickup_task(&self) -> Result<ClickupTaskResponseDto, ClickupError>;
    fn try_to_clickup_time_entries(&self) -> Result<ClickupTimeEntriesResponseDto, ClickupError>;
}

fn try_into_dto<'de, T>(text: &'de str) -> Result<T>
where
    T: serde::Deserialize<'de>,
{
    let dto = match serde_json::from_str::<T>(text) {
        Ok(dto) => dto,
        Err(e) => {
            let any_dto = serde_json::from_str::<Value>(text).unwrap();
            bail!(
                "Fail to deserialize. Error: {:#?}, Response text: {:#?}",
                e,
                any_dto
            );
        }
    };
    Ok(dto)
}

impl ParseClickupResponse for ClickupResponse {
    fn try_to_serde_json_value(&self) -> Result<Value, ClickupError> {
        if self.0.is_success() {
            Ok(try_into_dto::<Value>(self.0.text()).unwrap())
        } else {
            Err(try_into_dto::<ClickupError>(self.0.text()).unwrap())
        }
    }
    fn try_to_clickup_teams(&self) -> Result<ClickupTeamsResponseDto, ClickupError> {
        if self.0.is_success() {
            Ok(try_into_dto::<ClickupTeamsResponseDto>(self.0.text()).unwrap())
        } else {
            Err(try_into_dto::<ClickupError>(self.0.text()).unwrap())
        }
    }
    fn try_to_clickup_spaces(&self) -> Result<ClickupSpacesResponseDto, ClickupError> {
        if self.0.is_success() {
            Ok(try_into_dto::<ClickupSpacesResponseDto>(self.0.text()).unwrap())
        } else {
            Err(try_into_dto::<ClickupError>(self.0.text()).unwrap())
        }
    }
    fn try_to_clickup_folders(&self) -> Result<ClickupFoldersResponseDto, ClickupError> {
        if self.0.is_success() {
            Ok(try_into_dto::<ClickupFoldersResponseDto>(self.0.text()).unwrap())
        } else {
            Err(try_into_dto::<ClickupError>(self.0.text()).unwrap())
        }
    }
    fn try_to_clickup_lists(&self) -> Result<ClickupListsResponseDto, ClickupError> {
        if self.0.is_success() {
            Ok(try_into_dto::<ClickupListsResponseDto>(self.0.text()).unwrap())
        } else {
            Err(try_into_dto::<ClickupError>(self.0.text()).unwrap())
        }
    }
    fn try_to_clickup_list(&self) -> Result<ClickupListResponseDto, ClickupError> {
        if self.0.is_success() {
            Ok(try_into_dto::<ClickupListResponseDto>(self.0.text()).unwrap())
        } else {
            Err(try_into_dto::<ClickupError>(self.0.text()).unwrap())
        }
    }
    fn try_to_clickup_tasks(&self) -> Result<ClickupTasksResponseDto, ClickupError> {
        if self.0.is_success() {
            Ok(try_into_dto::<ClickupTasksResponseDto>(self.0.text()).unwrap())
        } else {
            Err(try_into_dto::<ClickupError>(self.0.text()).unwrap())
        }
    }
    fn try_to_clickup_task(&self) -> Result<ClickupTaskResponseDto, ClickupError> {
        if self.0.is_success() {
            Ok(try_into_dto::<ClickupTaskResponseDto>(self.0.text()).unwrap())
        } else {
            Err(try_into_dto::<ClickupError>(self.0.text()).unwrap())
        }
    }
    fn try_to_clickup_time_entries(&self) -> Result<ClickupTimeEntriesResponseDto, ClickupError> {
        if self.0.is_success() {
            Ok(try_into_dto::<ClickupTimeEntriesResponseDto>(self.0.text()).unwrap())
        } else {
            Err(try_into_dto::<ClickupError>(self.0.text()).unwrap())
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub(crate) struct ClickupError {
    pub ECODE: String,
    pub err: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ClickupTeamsResponseDto {
    pub teams: Vec<ClickupTeamResponseDto>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ClickupTeamResponseDto {
    pub id: String,
    pub members: Vec<ClickupUserResponseDto>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ClickupUserResponseDto {
    pub user: ClickupUserPropertyResponseDto,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ClickupUserPropertyResponseDto {
    pub id: i64,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ClickupSpacesResponseDto {
    pub spaces: Vec<ClickupSpaceResponseDto>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ClickupSpaceResponseDto {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ClickupFoldersResponseDto {
    pub folders: Vec<ClickupFolderResponseDto>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ClickupFolderResponseDto {
    pub id: String,
    pub name: String,
    pub space: ClickupSpaceResponseDto,
    pub lists: Vec<ClickupListResponseDto>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ClickupListsResponseDto {
    pub lists: Vec<ClickupListResponseDto>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ClickupListResponseDto {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ClickupTasksResponseDto {
    pub tasks: Vec<ClickupTaskResponseDto>,
}

impl From<ClickupTasksResponseDto> for ClickupTasks {
    fn from(dto: ClickupTasksResponseDto) -> Self {
        Self::new(dto.tasks.into_iter().map(|t| t.into()).collect())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ClickupTaskResponseDto {
    pub id: String,
    pub name: String,
    pub status: ClickupTaskStatusResponseDto,
    pub date_updated: String,
    pub time_spent: Option<i64>,
    pub list: ClickupListResponseDto,
    pub url: String,
}

impl From<ClickupTaskResponseDto> for ClickupTask {
    fn from(dto: ClickupTaskResponseDto) -> Self {
        Self::new(
            dto.id,
            dto.name,
            dto.url,
            dto.status.status,
            dto.list.name,
            None,
            dto.date_updated.as_str(),
        )
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ClickupTaskStatusResponseDto {
    pub status: String,
    pub color: String,
    pub r#type: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ClickupTimeEntriesResponseDto {
    pub data: Vec<ClickupTimeEntryResponseDto>,
}

impl From<ClickupTimeEntriesResponseDto> for ClickupTimeEntries {
    fn from(dto: ClickupTimeEntriesResponseDto) -> Self {
        Self::new(dto.data.into_iter().map(|te| te.into()).collect())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ClickupTimeEntryResponseDto {
    pub id: String,
    pub duration: String,
    pub start: String,
    pub end: String,
    pub task: ClickupTaskMiniResponseDto,
    pub task_url: String,
    pub task_location: ClickupTaskLocationResponseDto,
}

impl From<ClickupTimeEntryResponseDto> for ClickupTimeEntry {
    fn from(dto: ClickupTimeEntryResponseDto) -> ClickupTimeEntry {
        ClickupTimeEntry::new(
            dto.id,
            dto.duration.as_str(),
            dto.start.as_str(),
            dto.end.as_str(),
            dto.task.id,
            dto.task.name,
            dto.task_url,
            dto.task.status.status,
            dto.task_location.list_name,
        )
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ClickupTaskMiniResponseDto {
    pub id: String,
    pub name: String,
    pub status: ClickupTaskStatusResponseDto,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ClickupTaskLocationResponseDto {
    pub list_id: String,
    pub list_name: String,
    pub folder_id: String,
    pub folder_name: String,
    pub space_id: String,
    pub space_name: String,
}
