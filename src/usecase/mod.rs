mod aggregate_duration;

use crate::{domain::repository::Repositories, infra::repository_impl::RepositoryImpls};
pub(crate) use aggregate_duration::AggregateDurationUseCase;

pub(crate) trait Usecases {
    type Repositories: Repositories;

    fn aggregate_duration_use_case(&self) -> &AggregateDurationUseCase<Self::Repositories>;
}

#[derive(Debug, Clone)]
pub(crate) struct UsecaseImpls {
    aggregate_duration_use_case: AggregateDurationUseCase<RepositoryImpls>,
}

impl Usecases for UsecaseImpls {
    type Repositories = RepositoryImpls;

    fn aggregate_duration_use_case(&self) -> &AggregateDurationUseCase<Self::Repositories> {
        &self.aggregate_duration_use_case
    }
}

impl UsecaseImpls {
    pub(crate) fn new(repositories: RepositoryImpls) -> UsecaseImpls {
        let aggregate_duration_use_case = AggregateDurationUseCase::new(
            repositories.clickup_task_repository().to_owned(),
            repositories.clickup_time_entry_repository().to_owned(),
        );

        Self {
            aggregate_duration_use_case,
        }
    }
}
